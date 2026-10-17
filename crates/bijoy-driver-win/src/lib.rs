#![allow(dead_code)]

use bijoy_core::{BijoyEngine, BijoyMode};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::thread;

// Constants from Windows SDK
const WH_KEYBOARD_LL: i32 = 13;
const WM_KEYDOWN: u32 = 0x0100;
const WM_SYSKEYDOWN: u32 = 0x0104;
const WM_KEYUP: u32 = 0x0101;
const WM_SYSKEYUP: u32 = 0x0105;

const VK_BACK: u32 = 0x08;
const VK_TAB: u32 = 0x09;
const VK_RETURN: u32 = 0x0D;
const VK_SHIFT: u32 = 0x10;
const VK_CONTROL: u32 = 0x11;
const VK_MENU: u32 = 0x12; // Alt
const VK_CAPITAL: u32 = 0x14; // Caps Lock
const VK_ESCAPE: u32 = 0x1B;
const VK_SPACE: u32 = 0x20;
const VK_F12: u32 = 0x7B;

const INPUT_KEYBOARD: u32 = 1;
const KEYEVENTF_KEYUP: u32 = 0x0002;
const KEYEVENTF_UNICODE: u32 = 0x0004;

// FFI Types
type HHOOK = *mut std::ffi::c_void;
type HINSTANCE = *mut std::ffi::c_void;
type LRESULT = isize;
type WPARAM = usize;
type LPARAM = isize;
type HOOKPROC = unsafe extern "system" fn(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT;

#[repr(C)]
struct KBDLLHOOKSTRUCT {
    vk_code: u32,
    scan_code: u32,
    flags: u32,
    time: u32,
    dw_extra_info: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct KEYBDINPUT {
    w_vk: u16,
    w_scan: u16,
    dw_flags: u32,
    time: u32,
    dw_extra_info: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
union INPUT_UNION {
    ki: KEYBDINPUT,
    padding: [u8; 32],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct INPUT {
    type_: u32,
    u: INPUT_UNION,
}

#[repr(C)]
struct POINT {
    x: i32,
    y: i32,
}

#[repr(C)]
struct MSG {
    hwnd: *mut std::ffi::c_void,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    time: u32,
    pt: POINT,
}

#[link(name = "user32")]
#[link(name = "kernel32")]
extern "system" {
    fn SetWindowsHookExW(idHook: i32, lpfn: HOOKPROC, hmod: HINSTANCE, dwThreadId: u32) -> HHOOK;
    fn UnhookWindowsHookEx(hhk: HHOOK) -> i32;
    fn CallNextHookEx(hhk: HHOOK, nCode: i32, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    fn SendInput(cInputs: u32, pInputs: *const INPUT, cbSize: i32) -> u32;
    fn GetMessageW(lpMsg: *mut MSG, hWnd: *mut std::ffi::c_void, wMsgFilterMin: u32, wMsgFilterMax: u32) -> i32;
    fn TranslateMessage(lpMsg: *const MSG) -> i32;
    fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;
    fn PostThreadMessageW(idThread: u32, msg: u32, wParam: WPARAM, lParam: LPARAM) -> i32;
    fn GetKeyState(nVirtKey: i32) -> i16;
    fn GetAsyncKeyState(vKey: i32) -> i16;
    fn ToUnicode(
        wVirtKey: u32,
        wScanCode: u32,
        lpKeyState: *const u8,
        pwszBuff: *mut u16,
        cchBuff: i32,
        wFlags: u32,
    ) -> i32;
    fn GetKeyboardState(lpKeyState: *mut u8) -> i32;
    fn GetModuleHandleW(lpModuleName: *const u16) -> HINSTANCE;
    fn GetCurrentThreadId() -> u32;
}

// Global state for low-level hook callback
static HOOK_HANDLE: AtomicUsize = AtomicUsize::new(0);
static HOOK_THREAD_ID: AtomicUsize = AtomicUsize::new(0);
static IS_RUNNING: AtomicBool = AtomicBool::new(false);
static INJECTING: AtomicBool = AtomicBool::new(false);

static ENGINE: Mutex<Option<BijoyEngine>> = Mutex::new(None);
static LAST_BIJOY_MODE: Mutex<BijoyMode> = Mutex::new(BijoyMode::Unicode);

pub fn init_engine() {
    let mut lock = ENGINE.lock().unwrap();
    if lock.is_none() {
        *lock = Some(BijoyEngine::new());
    }
}

pub fn get_current_mode() -> BijoyMode {
    let lock = ENGINE.lock().unwrap();
    lock.as_ref().map(|e| e.mode).unwrap_or(BijoyMode::Unicode)
}

pub fn set_current_mode(mode: BijoyMode) {
    let mut lock = ENGINE.lock().unwrap();
    if let Some(engine) = lock.as_mut() {
        engine.set_mode(mode);
    }
    if mode != BijoyMode::English {
        let mut last_mode = LAST_BIJOY_MODE.lock().unwrap();
        *last_mode = mode;
    }
}

pub fn toggle_mode() {
    let current = get_current_mode();
    if current == BijoyMode::English {
        let last = *LAST_BIJOY_MODE.lock().unwrap();
        set_current_mode(last);
    } else {
        set_current_mode(BijoyMode::English);
    }
}

pub fn get_preview_info() -> Option<(String, Vec<String>)> {
    let lock = ENGINE.lock().unwrap();
    if let Some(engine) = lock.as_ref() {
        let (q, sugs) = engine.get_preview_info();
        if !q.is_empty() {
            return Some((q, sugs));
        }
    }
    None
}

pub fn is_hook_running() -> bool {
    IS_RUNNING.load(Ordering::SeqCst)
}

/// Start system-wide keyboard hook in a background thread
pub fn start_hook() -> Result<(), &'static str> {
    if is_hook_running() {
        return Ok(());
    }

    init_engine();

    thread::spawn(|| {
        unsafe {
            let thread_id = GetCurrentThreadId();
            HOOK_THREAD_ID.store(thread_id as usize, Ordering::SeqCst);

            let hook = SetWindowsHookExW(
                WH_KEYBOARD_LL,
                low_level_keyboard_proc,
                std::ptr::null_mut(),
                0,
            );

            if hook.is_null() {
                IS_RUNNING.store(false, Ordering::SeqCst);
                eprintln!("[Bijoy Windows Hook] Failed to install WH_KEYBOARD_LL hook");
                return;
            }

            HOOK_HANDLE.store(hook as usize, Ordering::SeqCst);
            IS_RUNNING.store(true, Ordering::SeqCst);
            println!("[Bijoy Windows Hook] Hook installed successfully. Ready to type Bangla.");

            let mut msg: MSG = std::mem::zeroed();
            while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            UnhookWindowsHookEx(hook);
            HOOK_HANDLE.store(0, Ordering::SeqCst);
            IS_RUNNING.store(false, Ordering::SeqCst);
            println!("[Bijoy Windows Hook] Hook uninstalled.");
        }
    });

    Ok(())
}

/// Stop system-wide keyboard hook
pub fn stop_hook() {
    if !is_hook_running() {
        return;
    }

    let thread_id = HOOK_THREAD_ID.load(Ordering::SeqCst) as u32;
    if thread_id != 0 {
        unsafe {
            const WM_QUIT: u32 = 0x0012;
            PostThreadMessageW(thread_id, WM_QUIT, 0, 0);
        }
    }
    IS_RUNNING.store(false, Ordering::SeqCst);
}

const BIJOY_EXTRA_INFO: usize = 0x367C2024;
static KEY_CONSUMED: [AtomicBool; 256] = [const { AtomicBool::new(false) }; 256];

/// Inject Backspaces and Unicode characters in a single atomic call to SendInput
unsafe fn inject_output(delete_count: usize, text: &str) {
    let utf16_units: Vec<u16> = text.encode_utf16().collect();
    let total_events = delete_count * 2 + utf16_units.len() * 2;
    if total_events == 0 {
        return;
    }
    let mut inputs: Vec<INPUT> = Vec::with_capacity(total_events);

    // 1. Backspaces (delete_count)
    for _ in 0..delete_count {
        inputs.push(INPUT {
            type_: INPUT_KEYBOARD,
            u: INPUT_UNION {
                ki: KEYBDINPUT {
                    w_vk: VK_BACK as u16,
                    w_scan: 0,
                    dw_flags: 0,
                    time: 0,
                    dw_extra_info: BIJOY_EXTRA_INFO,
                },
            },
        });
        inputs.push(INPUT {
            type_: INPUT_KEYBOARD,
            u: INPUT_UNION {
                ki: KEYBDINPUT {
                    w_vk: VK_BACK as u16,
                    w_scan: 0,
                    dw_flags: KEYEVENTF_KEYUP,
                    time: 0,
                    dw_extra_info: BIJOY_EXTRA_INFO,
                },
            },
        });
    }

    // 2. Unicode characters
    for &code in &utf16_units {
        inputs.push(INPUT {
            type_: INPUT_KEYBOARD,
            u: INPUT_UNION {
                ki: KEYBDINPUT {
                    w_vk: 0,
                    w_scan: code,
                    dw_flags: KEYEVENTF_UNICODE,
                    time: 0,
                    dw_extra_info: BIJOY_EXTRA_INFO,
                },
            },
        });
        inputs.push(INPUT {
            type_: INPUT_KEYBOARD,
            u: INPUT_UNION {
                ki: KEYBDINPUT {
                    w_vk: 0,
                    w_scan: code,
                    dw_flags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                    time: 0,
                    dw_extra_info: BIJOY_EXTRA_INFO,
                },
            },
        });
    }

    SendInput(inputs.len() as u32, inputs.as_ptr(), std::mem::size_of::<INPUT>() as i32);
}

/// The Low-Level Keyboard Hook Procedure
unsafe extern "system" fn low_level_keyboard_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let hook = HOOK_HANDLE.load(Ordering::SeqCst) as HHOOK;

    if code < 0 {
        return CallNextHookEx(hook, code, wparam, lparam);
    }

    let kbd = &*(lparam as *const KBDLLHOOKSTRUCT);

    // CRITICAL: Ignore synthetic events injected by ourselves or other input injection APIs
    const LLKHF_INJECTED: u32 = 0x00000010;
    if (kbd.flags & LLKHF_INJECTED) != 0 || kbd.dw_extra_info == BIJOY_EXTRA_INFO {
        return CallNextHookEx(hook, code, wparam, lparam);
    }

    let is_down = wparam as u32 == WM_KEYDOWN || wparam as u32 == WM_SYSKEYDOWN;
    let is_up = wparam as u32 == WM_KEYUP || wparam as u32 == WM_SYSKEYUP;

    let ctrl_pressed = (GetKeyState(VK_CONTROL as i32) as u16 & 0x8000) != 0;
    let alt_pressed = (GetKeyState(VK_MENU as i32) as u16 & 0x8000) != 0;
    let shift_pressed = (GetKeyState(VK_SHIFT as i32) as u16 & 0x8000) != 0;

    // Check Hotkeys on Key Down:
    // Ctrl + Alt + V => Unicode Mode
    // Ctrl + Alt + B => Classic Mode
    // Ctrl + Alt + E => English Mode
    // F12 => Toggle English <-> Active Bangla Layout
    if is_down {
        if ctrl_pressed && alt_pressed {
            match kbd.vk_code {
                0x56 => { // 'V'
                    set_current_mode(BijoyMode::Unicode);
                    println!("[Bijoy] Switched to Bijoy UNICODE mode (Ctrl+Alt+V)");
                    return 1;
                }
                0x42 => { // 'B'
                    set_current_mode(BijoyMode::Classic);
                    println!("[Bijoy] Switched to Bijoy CLASSIC / SutonnyMJ mode (Ctrl+Alt+B)");
                    return 1;
                }
                0x45 => { // 'E'
                    set_current_mode(BijoyMode::English);
                    println!("[Bijoy] Switched to ENGLISH mode (Ctrl+Alt+E)");
                    return 1;
                }
                _ => {}
            }
        } else if kbd.vk_code == VK_F12 {
            toggle_mode();
            println!("[Bijoy] Toggled mode to: {:?}", get_current_mode());
            return 1;
        }
    }

    let current_mode = get_current_mode();
    if current_mode == BijoyMode::English {
        return CallNextHookEx(hook, code, wparam, lparam);
    }

    // If Ctrl or Alt or Windows key is down (shortcuts like Ctrl+C, Ctrl+V, Alt+Tab), pass through!
    if ctrl_pressed || alt_pressed {
        return CallNextHookEx(hook, code, wparam, lparam);
    }

    // Handle Backspace
    if kbd.vk_code == VK_BACK {
        if is_down {
            let mut lock = ENGINE.lock().unwrap();
            if let Some(engine) = lock.as_mut() {
                engine.process_char('\x08');
            }
        }
        return CallNextHookEx(hook, code, wparam, lparam);
    }

    // Special keys like Enter, Tab, Escape, Arrows, Function keys: reset engine & pass through
    if kbd.vk_code == VK_RETURN
        || kbd.vk_code == VK_TAB
        || kbd.vk_code == VK_ESCAPE
        || (kbd.vk_code >= 0x21 && kbd.vk_code <= 0x28) // PgUp, PgDn, End, Home, Left, Up, Right, Down
        || (kbd.vk_code >= 0x70 && kbd.vk_code <= 0x87) // F1-F24
        || kbd.vk_code == VK_CAPITAL
        || kbd.vk_code == VK_SHIFT
        || kbd.vk_code == VK_CONTROL
        || kbd.vk_code == VK_MENU
    {
        if is_down {
            let mut lock = ENGINE.lock().unwrap();
            if let Some(engine) = lock.as_mut() {
                engine.reset();
            }
        }
        return CallNextHookEx(hook, code, wparam, lparam);
    }

    // Key-up handling: if key down was consumed, consume key up cleanly
    let vk_idx = (kbd.vk_code & 0xFF) as usize;
    if is_up {
        if KEY_CONSUMED[vk_idx].swap(false, Ordering::SeqCst) {
            return 1;
        }
        return CallNextHookEx(hook, code, wparam, lparam);
    }

    // Translate virtual key to Unicode char
    let mut key_state = [0u8; 256];
    GetKeyboardState(key_state.as_mut_ptr());
    if shift_pressed {
        key_state[VK_SHIFT as usize] = 0x80;
    } else {
        key_state[VK_SHIFT as usize] = 0;
    }

    let mut buf = [0u16; 8];
    let count = ToUnicode(
        kbd.vk_code,
        kbd.scan_code,
        key_state.as_ptr(),
        buf.as_mut_ptr(),
        buf.len() as i32,
        0,
    );

    if count <= 0 {
        return CallNextHookEx(hook, code, wparam, lparam);
    }

    let input_char = match String::from_utf16(&buf[..count as usize]) {
        Ok(s) => s.chars().next().unwrap_or('\0'),
        Err(_) => return CallNextHookEx(hook, code, wparam, lparam),
    };

    if input_char == '\0' {
        return CallNextHookEx(hook, code, wparam, lparam);
    }

    if is_down {
        let mut lock = ENGINE.lock().unwrap();
        if let Some(engine) = lock.as_mut() {
            let out = engine.process_char(input_char);
            if out.consumed {
                KEY_CONSUMED[vk_idx].store(true, Ordering::SeqCst);
                if out.delete_count > 0 || !out.insert_text.is_empty() {
                    inject_output(out.delete_count, &out.insert_text);
                }
                return 1; // Block original key
            }
        }
    }

    CallNextHookEx(hook, code, wparam, lparam)
}
