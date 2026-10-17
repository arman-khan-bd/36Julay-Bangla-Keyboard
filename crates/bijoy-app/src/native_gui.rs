//! Pure Native Windows GUI for Bijoy 36July Bangla Keyboard.
//! Implements the floating top bar (Image 1), floating preview window (Image 2),
//! native converter dialog, virtual keyboard, and system tray.
//! 100% Native Win32 desktop application — NO web browser involved!

#![cfg(target_os = "windows")]

use crate::win32_api::*;
use bijoy_core::{avro_to_36july, bijoy_36july_to_avro, BijoyMode};
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
struct SafeHwnd(HWND);
unsafe impl Send for SafeHwnd {}
unsafe impl Sync for SafeHwnd {}

static TOP_BAR_HWND: Mutex<Option<SafeHwnd>> = Mutex::new(None);
static PREVIEW_HWND: Mutex<Option<SafeHwnd>> = Mutex::new(None);
static CONVERTER_HWND: Mutex<Option<SafeHwnd>> = Mutex::new(None);
static VK_HWND: Mutex<Option<SafeHwnd>> = Mutex::new(None);

static CURRENT_MODE_CACHED: Mutex<BijoyMode> = Mutex::new(BijoyMode::Unicode);
static PREVIEW_QUERY: Mutex<String> = Mutex::new(String::new());
static PREVIEW_SUGGESTIONS: Mutex<Vec<String>> = Mutex::new(Vec::new());
static PREVIEW_PINNED: AtomicBool = AtomicBool::new(false);
static MOUSE_WAS_DOWN: AtomicBool = AtomicBool::new(false);

/// Helper to create high-quality Bengali font with Kalpurush (installed on Windows)
pub unsafe fn create_bengali_font(height: i32, weight: i32) -> HFONT {
    CreateFontW(
        height, 0, 0, 0, weight, 0, 0, 0,
        1, // DEFAULT_CHARSET
        0, // OUT_DEFAULT_PRECIS
        0, // CLIP_DEFAULT_PRECIS
        5, // CLEARTYPE_QUALITY
        0, // DEFAULT_PITCH
        to_wide_str("Kalpurush").as_ptr(),
    )
}

unsafe fn hide_preview_if_not_pinned() {
    let mut q_lock = PREVIEW_QUERY.lock().unwrap();
    let mut s_lock = PREVIEW_SUGGESTIONS.lock().unwrap();
    if !q_lock.is_empty() {
        q_lock.clear();
        s_lock.clear();
        if let Some(prev_hwnd) = *PREVIEW_HWND.lock().unwrap() {
            InvalidateRect(prev_hwnd.0, std::ptr::null(), 1);
        }
    }
    if let Some(prev_hwnd) = *PREVIEW_HWND.lock().unwrap() {
        if IsWindowVisible(prev_hwnd.0) != 0 && !PREVIEW_PINNED.load(std::sync::atomic::Ordering::SeqCst) {
            ShowWindow(prev_hwnd.0, SW_HIDE);
        }
    }
}

const TRAY_ICON_ID: u32 = 1001;
const WM_TRAYICON: u32 = WM_USER + 101;
const TIMER_SYNC_ID: usize = 2001;

// Menu IDs
const ID_MENU_ENGLISH: usize = 3001;
const ID_MENU_UNICODE: usize = 3002;
const ID_MENU_CLASSIC: usize = 3003;
const ID_MENU_AVRO: usize = 3004;
const ID_MENU_KEYBOARD: usize = 3005;
const ID_MENU_PREVIEW: usize = 3006;
const ID_MENU_CONVERTER: usize = 3007;
const ID_MENU_HELP: usize = 3008;
const ID_MENU_EXIT: usize = 3009;

// Converter control IDs
const ID_EDIT_AVRO: usize = 4001;
const ID_EDIT_BIJOY: usize = 4002;
const ID_BTN_TO_BIJOY: usize = 4003;
const ID_BTN_TO_AVRO: usize = 4004;
const ID_BTN_SWAP: usize = 4005;
const ID_BTN_COPY: usize = 4006;
const ID_BTN_CLEAR: usize = 4007;

/// Initialize and run the Native Windows GUI
pub fn run_native_gui() {
    unsafe {
        let h_instance = GetModuleHandleW(std::ptr::null());

        // Default query and suggestions start empty
        *PREVIEW_QUERY.lock().unwrap() = String::new();
        *PREVIEW_SUGGESTIONS.lock().unwrap() = Vec::new();

        // 1. Register Window Classes
        register_classes(h_instance);

        // 2. Create Top Floating Keyboard Bar (Image 1)
        let top_bar = create_top_bar_window(h_instance);
        *TOP_BAR_HWND.lock().unwrap() = Some(SafeHwnd(top_bar));

        // 3. Create Preview Window (Image 2 - starts hidden until typing starts)
        let preview_win = create_preview_window(h_instance);
        *PREVIEW_HWND.lock().unwrap() = Some(SafeHwnd(preview_win));

        // 4. Create System Tray Icon
        setup_tray_icon(top_bar, h_instance);

        // 5. Start a periodic 30ms timer to synchronize mode and auto show/hide preview
        SetTimer(top_bar, TIMER_SYNC_ID, 30, None);

        println!("✓ Native Windows Top Bar (Image 1) & Preview Window (Image 2) active!");
        println!("  Press Ctrl+Alt+V (Unicode), Ctrl+Alt+B (Classic), F12 (Avro), Ctrl+Alt+E (English)");

        // 6. Win32 Message Loop
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        // Clean up tray icon on exit
        remove_tray_icon(top_bar);
    }
}

unsafe fn register_classes(h_instance: HINSTANCE) {
    let arrow_cursor = LoadCursorW(std::ptr::null_mut(), IDC_ARROW);

    // Class for Top Bar
    let class_top = to_wide_str("BijoyTopBarClass");
    let wc_top = WNDCLASSEXW {
        cb_size: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfn_wnd_proc: Some(top_bar_wnd_proc),
        cb_cls_extra: 0,
        cb_wnd_extra: 0,
        h_instance,
        h_icon: std::ptr::null_mut(),
        h_cursor: arrow_cursor,
        hbr_background: std::ptr::null_mut(),
        lpsz_menu_name: std::ptr::null(),
        lpsz_class_name: class_top.as_ptr(),
        h_icon_sm: std::ptr::null_mut(),
    };
    RegisterClassExW(&wc_top);

    // Class for Preview Window
    let class_preview = to_wide_str("BijoyPreviewClass");
    let wc_preview = WNDCLASSEXW {
        cb_size: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfn_wnd_proc: Some(preview_wnd_proc),
        cb_cls_extra: 0,
        cb_wnd_extra: 0,
        h_instance,
        h_icon: std::ptr::null_mut(),
        h_cursor: arrow_cursor,
        hbr_background: std::ptr::null_mut(),
        lpsz_menu_name: std::ptr::null(),
        lpsz_class_name: class_preview.as_ptr(),
        h_icon_sm: std::ptr::null_mut(),
    };
    RegisterClassExW(&wc_preview);

    // Class for Converter Window
    let class_conv = to_wide_str("BijoyConverterClass");
    let wc_conv = WNDCLASSEXW {
        cb_size: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfn_wnd_proc: Some(converter_wnd_proc),
        cb_cls_extra: 0,
        cb_wnd_extra: 0,
        h_instance,
        h_icon: std::ptr::null_mut(),
        h_cursor: arrow_cursor,
        hbr_background: CreateSolidBrush(rgb(240, 242, 245)),
        lpsz_menu_name: std::ptr::null(),
        lpsz_class_name: class_conv.as_ptr(),
        h_icon_sm: std::ptr::null_mut(),
    };
    RegisterClassExW(&wc_conv);

    // Class for Virtual Keyboard Window
    let class_vk = to_wide_str("BijoyVirtualKeyboardClass");
    let wc_vk = WNDCLASSEXW {
        cb_size: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfn_wnd_proc: Some(vk_wnd_proc),
        cb_cls_extra: 0,
        cb_wnd_extra: 0,
        h_instance,
        h_icon: std::ptr::null_mut(),
        h_cursor: arrow_cursor,
        hbr_background: CreateSolidBrush(rgb(18, 24, 38)),
        lpsz_menu_name: std::ptr::null(),
        lpsz_class_name: class_vk.as_ptr(),
        h_icon_sm: std::ptr::null_mut(),
    };
    RegisterClassExW(&wc_vk);
}

// ============================================================================
// 1. TOP FLOATING KEYBOARD BAR (IMAGE 1)
// ============================================================================
unsafe fn create_top_bar_window(h_instance: HINSTANCE) -> HWND {
    let screen_w = GetSystemMetrics(SM_CXSCREEN);
    let bar_w = 430;
    let bar_h = 36;
    let bar_x = (screen_w - bar_w) / 2;
    let bar_y = 6;

    let class_name = to_wide_str("BijoyTopBarClass");
    let title = to_wide_str("July36 বাংলা কীবোর্ড");

    let hwnd = CreateWindowExW(
        WS_EX_TOPMOST | WS_EX_TOOLWINDOW,
        class_name.as_ptr(),
        title.as_ptr(),
        WS_POPUP | WS_VISIBLE,
        bar_x,
        bar_y,
        bar_w,
        bar_h,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        h_instance,
        std::ptr::null_mut(),
    );

    ShowWindow(hwnd, SW_SHOW);
    UpdateWindow(hwnd);
    hwnd
}

unsafe extern "system" fn top_bar_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let hdc = BeginPaint(hwnd, &mut ps);
            let mut rc: RECT = std::mem::zeroed();
            GetClientRect(hwnd, &mut rc);

            // Dark sleek gradient-like background (Image 1)
            let brush_bg = CreateSolidBrush(rgb(18, 24, 38));
            FillRect(hdc, &rc, brush_bg);
            DeleteObject(brush_bg as *mut c_void);

            // Subtle border around bar
            let brush_border = CreateSolidBrush(rgb(56, 75, 105));
            let old_brush = SelectObject(hdc, brush_border as *mut c_void);
            RoundRect(hdc, rc.left, rc.top, rc.right, rc.bottom, 8, 8);
            SelectObject(hdc, old_brush);
            DeleteObject(brush_border as *mut c_void);

            SetBkMode(hdc, TRANSPARENT);

            // 1. Golden 'অ' Logo on Left (Image 1)
            let font_logo = create_bengali_font(24, 800);
            let old_font = SelectObject(hdc, font_logo as *mut c_void);
            SetTextColor(hdc, rgb(246, 173, 85)); // Golden color

            let mut logo_rc = RECT { left: 8, top: 2, right: 34, bottom: 34 };
            let logo_text = to_wide_str("অ");
            DrawTextW(hdc, logo_text.as_ptr(), -1, &mut logo_rc, DT_CENTER | DT_VCENTER | DT_SINGLELINE);
            SelectObject(hdc, old_font);
            DeleteObject(font_logo as *mut c_void);

            // 2. Layout Selector Area (Image 1: "English ▼")
            let mode = *CURRENT_MODE_CACHED.lock().unwrap();
            let mode_label = match mode {
                BijoyMode::Unicode => "July36 ▼",
                BijoyMode::Classic => "বিজয় ক্লাসিক ▼",
                BijoyMode::Avro => "অভ্র ফোনেটিক ▼",
                BijoyMode::English => "English ▼",
            };

            let font_text = CreateFontW(
                15, 0, 0, 0, 600, 0, 0, 0, 1, 0, 0, 5, 0,
                to_wide_str("Segoe UI").as_ptr(),
            );
            let old_font2 = SelectObject(hdc, font_text as *mut c_void);
            SetTextColor(hdc, rgb(238, 242, 255)); // Crisp white

            let mut layout_rc = RECT { left: 40, top: 4, right: 165, bottom: 32 };
            let label_w = to_wide_str(mode_label);
            DrawTextW(hdc, label_w.as_ptr(), -1, &mut layout_rc, DT_LEFT | DT_VCENTER | DT_SINGLELINE);

            // 3. Icons on Right matching Image 1:
            // ⌨️ Keyboard (x: 180), 👔/📝 Preview (x: 215), ⚙️ Settings (x: 250), 🌐 Web/Converter (x: 285), ❓ Help (x: 320), ℹ️ Info (x: 355), ✕ Exit (x: 390)
            let icons = [
                (175, 205, "⌨"),
                (210, 240, "📝"),
                (245, 275, "⚙"),
                (280, 310, "🌐"),
                (315, 345, "❓"),
                (350, 380, "ℹ"),
                (385, 415, "✕"),
            ];

            let font_icon = CreateFontW(
                16, 0, 0, 0, 500, 0, 0, 0, 1, 0, 0, 5, 0,
                to_wide_str("Segoe UI Emoji").as_ptr(),
            );
            SelectObject(hdc, font_icon as *mut c_void);
            SetTextColor(hdc, rgb(180, 200, 230));

            for (x1, x2, symbol) in icons {
                let mut btn_rc = RECT { left: x1, top: 4, right: x2, bottom: 32 };
                let sym_w = to_wide_str(symbol);
                DrawTextW(hdc, sym_w.as_ptr(), -1, &mut btn_rc, DT_CENTER | DT_VCENTER | DT_SINGLELINE);
            }

            SelectObject(hdc, old_font2);
            DeleteObject(font_text as *mut c_void);
            DeleteObject(font_icon as *mut c_void);

            EndPaint(hwnd, &ps);
            0
        }
        WM_LBUTTONDOWN => {
            let x = (lparam & 0xFFFF) as i16 as i32;
            let _y = ((lparam >> 16) & 0xFFFF) as i16 as i32;

            // If clicked on layout selector (x: 35..165), open layout menu
            if x >= 35 && x <= 165 {
                show_layout_popup_menu(hwnd);
                return 0;
            }

            // If clicked on icons:
            if x >= 175 && x <= 205 {
                // ⌨️ Keyboard toggle
                show_virtual_keyboard();
                return 0;
            }
            if x >= 210 && x <= 240 {
                // 📝 Preview window toggle
                toggle_preview_window();
                return 0;
            }
            if x >= 245 && x <= 275 {
                // ⚙️ Settings
                show_settings_dialog(hwnd);
                return 0;
            }
            if x >= 280 && x <= 310 {
                // 🌐 Converter window
                show_converter_window();
                return 0;
            }
            if x >= 315 && x <= 345 {
                // ❓ Help
                show_help_dialog(hwnd);
                return 0;
            }
            if x >= 350 && x <= 380 {
                // ℹ Info
                show_about_dialog(hwnd);
                return 0;
            }
            if x >= 385 && x <= 425 {
                // ✕ Minimize to tray
                ShowWindow(hwnd, SW_HIDE);
                return 0;
            }

            // Click and drag support for the bar anywhere else!
            ReleaseCapture();
            SendMessageW(hwnd, WM_NCLBUTTONDOWN, HTCAPTION as WPARAM, 0);
            0
        }
        WM_RBUTTONUP => {
            show_tray_popup_menu(hwnd);
            0
        }
        WM_TIMER => {
            if wparam == TIMER_SYNC_ID {
                // 1. Sync keyboard layout mode
                let actual_mode = bijoy_driver_win::get_current_mode();
                let mut cached = CURRENT_MODE_CACHED.lock().unwrap();
                if *cached != actual_mode {
                    *cached = actual_mode;
                    InvalidateRect(hwnd, std::ptr::null(), 1);
                }

                // 2. Click outside detection to auto-hide preview window
                let lbtn = (GetAsyncKeyState(VK_LBUTTON) as u16 & 0x8000) != 0;
                let rbtn = (GetAsyncKeyState(VK_RBUTTON) as u16 & 0x8000) != 0;
                let mbtn = (GetAsyncKeyState(VK_MBUTTON) as u16 & 0x8000) != 0;
                let mouse_down = lbtn || rbtn || mbtn;
                let was_down = MOUSE_WAS_DOWN.swap(mouse_down, std::sync::atomic::Ordering::SeqCst);

                if mouse_down && !was_down {
                    if let Some(prev_h) = *PREVIEW_HWND.lock().unwrap() {
                        let prev_hwnd = prev_h.0;
                        if IsWindowVisible(prev_hwnd) != 0 && !PREVIEW_PINNED.load(std::sync::atomic::Ordering::SeqCst) {
                            let mut pt: POINT = std::mem::zeroed();
                            GetCursorPos(&mut pt);

                            let mut rc_prev: RECT = std::mem::zeroed();
                            GetWindowRect(prev_hwnd, &mut rc_prev);

                            let mut rc_bar: RECT = std::mem::zeroed();
                            GetWindowRect(hwnd, &mut rc_bar);

                            let in_prev = pt_in_rect(&rc_prev, pt);
                            let in_bar = pt_in_rect(&rc_bar, pt);

                            if !in_prev && !in_bar {
                                ShowWindow(prev_hwnd, SW_HIDE);
                                PREVIEW_QUERY.lock().unwrap().clear();
                                PREVIEW_SUGGESTIONS.lock().unwrap().clear();
                                bijoy_driver_win::reset_engine();
                            }
                        }
                    }
                }

                // 3. Real-time preview query & suggestion update
                let info = bijoy_driver_win::get_preview_info();
                if let Some((query, sugs)) = info {
                    if !query.is_empty() {
                        let mut q_lock = PREVIEW_QUERY.lock().unwrap();
                        let mut s_lock = PREVIEW_SUGGESTIONS.lock().unwrap();
                        let changed = *q_lock != query;
                        if changed {
                            *q_lock = query;
                            *s_lock = sugs;
                            if let Some(prev_hwnd) = *PREVIEW_HWND.lock().unwrap() {
                                InvalidateRect(prev_hwnd.0, std::ptr::null(), 1);
                            }
                        }
                        if let Some(prev_hwnd) = *PREVIEW_HWND.lock().unwrap() {
                            if IsWindowVisible(prev_hwnd.0) == 0 {
                                ShowWindow(prev_hwnd.0, SW_SHOWNA);
                            }
                        }
                    } else {
                        hide_preview_if_not_pinned();
                    }
                } else {
                    hide_preview_if_not_pinned();
                }
            }
            0
        }
        WM_TRAYICON => {
            if lparam as u32 == WM_LBUTTONUP {
                ShowWindow(hwnd, SW_SHOW);
                SetForegroundWindow(hwnd);
            } else if lparam as u32 == WM_RBUTTONUP {
                show_tray_popup_menu(hwnd);
            }
            0
        }
        WM_COMMAND => {
            let cmd_id = wparam & 0xFFFF;
            match cmd_id {
                ID_MENU_ENGLISH => {
                    bijoy_driver_win::set_current_mode(BijoyMode::English);
                    InvalidateRect(hwnd, std::ptr::null(), 1);
                }
                ID_MENU_UNICODE => {
                    bijoy_driver_win::set_current_mode(BijoyMode::Unicode);
                    InvalidateRect(hwnd, std::ptr::null(), 1);
                }
                ID_MENU_CLASSIC => {
                    bijoy_driver_win::set_current_mode(BijoyMode::Classic);
                    InvalidateRect(hwnd, std::ptr::null(), 1);
                }
                ID_MENU_AVRO => {
                    bijoy_driver_win::set_current_mode(BijoyMode::Avro);
                    InvalidateRect(hwnd, std::ptr::null(), 1);
                }
                ID_MENU_KEYBOARD => show_virtual_keyboard(),
                ID_MENU_PREVIEW => toggle_preview_window(),
                ID_MENU_CONVERTER => show_converter_window(),
                ID_MENU_HELP => show_help_dialog(hwnd),
                ID_MENU_EXIT => {
                    PostQuitMessage(0);
                }
                _ => {}
            }
            0
        }
        WM_DESTROY => {
            KillTimer(hwnd, TIMER_SYNC_ID);
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

// ============================================================================
// 2. FLOATING PREVIEW WINDOW (IMAGE 2)
// ============================================================================
unsafe fn create_preview_window(h_instance: HINSTANCE) -> HWND {
    let screen_w = GetSystemMetrics(SM_CXSCREEN);
    let win_w = 210;
    let win_h = 168;
    let win_x = screen_w - win_w - 30;
    let win_y = 50;

    let class_name = to_wide_str("BijoyPreviewClass");
    let title = to_wide_str("Preview Window");

    let hwnd = CreateWindowExW(
        WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
        class_name.as_ptr(),
        title.as_ptr(),
        WS_POPUP,
        win_x,
        win_y,
        win_w,
        win_h,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        h_instance,
        std::ptr::null_mut(),
    );

    // Initially start hidden — appears automatically on typing anywhere!
    ShowWindow(hwnd, SW_HIDE);
    hwnd
}

unsafe extern "system" fn preview_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let hdc = BeginPaint(hwnd, &mut ps);
            let mut rc: RECT = std::mem::zeroed();
            GetClientRect(hwnd, &mut rc);

            // 1. Title bar (Image 2: "📝 Preview Window  📌")
            let title_rc = RECT { left: 0, top: 0, right: rc.right, bottom: 26 };
            let brush_title = CreateSolidBrush(rgb(235, 237, 240));
            FillRect(hdc, &title_rc, brush_title);
            DeleteObject(brush_title as *mut c_void);

            SetBkMode(hdc, TRANSPARENT);
            let font_title = CreateFontW(13, 0, 0, 0, 600, 0, 0, 0, 1, 0, 0, 5, 0, to_wide_str("Segoe UI").as_ptr());
            let old_font = SelectObject(hdc, font_title as *mut c_void);
            SetTextColor(hdc, rgb(30, 41, 59));

            let mut t_rc = RECT { left: 8, top: 0, right: rc.right - 24, bottom: 26 };
            let t_text = to_wide_str("📝 Preview Window");
            DrawTextW(hdc, t_text.as_ptr(), -1, &mut t_rc, DT_LEFT | DT_VCENTER | DT_SINGLELINE);

            let mut pin_rc = RECT { left: rc.right - 24, top: 0, right: rc.right - 4, bottom: 26 };
            let pin_text = to_wide_str(if PREVIEW_PINNED.load(std::sync::atomic::Ordering::SeqCst) { "📍" } else { "📌" });
            DrawTextW(hdc, pin_text.as_ptr(), -1, &mut pin_rc, DT_CENTER | DT_VCENTER | DT_SINGLELINE);

            // 2. Query Row (Image 2: bold query on pale yellow background)
            let query_rc = RECT { left: 0, top: 26, right: rc.right, bottom: 58 };
            let brush_query = CreateSolidBrush(rgb(255, 255, 224)); // Pale yellow (Image 2)
            FillRect(hdc, &query_rc, brush_query);
            DeleteObject(brush_query as *mut c_void);

            let font_query = CreateFontW(17, 0, 0, 0, 700, 0, 0, 0, 1, 0, 0, 5, 0, to_wide_str("Segoe UI").as_ptr());
            SelectObject(hdc, font_query as *mut c_void);
            SetTextColor(hdc, rgb(0, 0, 0));

            let query = PREVIEW_QUERY.lock().unwrap();
            let mut q_rc = RECT { left: 8, top: 26, right: rc.right - 8, bottom: 58 };
            let q_w = to_wide_str(&query);
            DrawTextW(hdc, q_w.as_ptr(), -1, &mut q_rc, DT_LEFT | DT_VCENTER | DT_SINGLELINE);
            DeleteObject(font_query as *mut c_void);

            // 3. Suggestions List in Crisp Bengali Kalpurush font (No tofus/boxes!)
            let suggestions = PREVIEW_SUGGESTIONS.lock().unwrap().clone();
            let font_sug = create_bengali_font(18, 600);
            SelectObject(hdc, font_sug as *mut c_void);

            for (idx, sug) in suggestions.iter().take(4).enumerate() {
                let y1 = 58 + (idx as i32 * 26);
                let y2 = y1 + 26;
                let item_rc = RECT { left: 0, top: y1, right: rc.right, bottom: y2 };

                if idx == 0 {
                    // Selected item in royal blue (Image 2)
                    let brush_sel = CreateSolidBrush(rgb(0, 120, 215));
                    FillRect(hdc, &item_rc, brush_sel);
                    DeleteObject(brush_sel as *mut c_void);
                    SetTextColor(hdc, rgb(255, 255, 255));
                } else {
                    let brush_item = CreateSolidBrush(rgb(255, 255, 255));
                    FillRect(hdc, &item_rc, brush_item);
                    DeleteObject(brush_item as *mut c_void);
                    SetTextColor(hdc, rgb(20, 20, 20));
                }

                let mut text_rc = RECT { left: 8, top: y1, right: rc.right - 8, bottom: y2 };
                let item_text = to_wide_str(&format!("{}. {}", idx + 1, sug));
                DrawTextW(hdc, item_text.as_ptr(), -1, &mut text_rc, DT_LEFT | DT_VCENTER | DT_SINGLELINE);
            }

            SelectObject(hdc, old_font);
            DeleteObject(font_title as *mut c_void);
            DeleteObject(font_sug as *mut c_void);

            EndPaint(hwnd, &ps);
            0
        }
        WM_LBUTTONDOWN => {
            let x = (lparam & 0xFFFF) as i16 as i32;
            let y = ((lparam >> 16) & 0xFFFF) as i16 as i32;
            if y <= 26 {
                let mut rc: RECT = std::mem::zeroed();
                GetClientRect(hwnd, &mut rc);
                if x >= rc.right - 26 {
                    let pinned = PREVIEW_PINNED.fetch_xor(true, std::sync::atomic::Ordering::SeqCst);
                    let flag = if !pinned { HWND_TOPMOST } else { HWND_NOTOPMOST };
                    SetWindowPos(hwnd, flag, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW);
                    InvalidateRect(hwnd, std::ptr::null(), 1);
                    return 0;
                }
                // Drag preview window
                ReleaseCapture();
                SendMessageW(hwnd, WM_NCLBUTTONDOWN, HTCAPTION as WPARAM, 0);
                return 0;
            } else if y >= 58 {
                let idx = ((y - 58) / 26) as usize;
                let sugs = PREVIEW_SUGGESTIONS.lock().unwrap().clone();
                if idx < sugs.len() {
                    bijoy_driver_win::select_suggestion(idx);
                    ShowWindow(hwnd, SW_HIDE);
                    PREVIEW_QUERY.lock().unwrap().clear();
                    PREVIEW_SUGGESTIONS.lock().unwrap().clear();
                    return 0;
                }
            }
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

// ============================================================================
// 3. AVRO ⇄ 36JULY (BIJOY) CONVERTER WINDOW
// ============================================================================
pub fn show_converter_window() {
    unsafe {
        let mut lock = CONVERTER_HWND.lock().unwrap();
        if let Some(h) = *lock {
            let hwnd = h.0;
            ShowWindow(hwnd, SW_SHOWNORMAL);
            SetForegroundWindow(hwnd);
            return;
        }

        let h_instance = GetModuleHandleW(std::ptr::null());
        let screen_w = GetSystemMetrics(SM_CXSCREEN);
        let screen_h = GetSystemMetrics(SM_CYSCREEN);
        let w = 660;
        let h = 480;
        let x = (screen_w - w) / 2;
        let y = (screen_h - h) / 2;

        let class_name = to_wide_str("BijoyConverterClass");
        let title = to_wide_str("July36 - অভ্র ⇄ ৩৬জুলাই কনভার্টার | BDS 1738");

        let hwnd = CreateWindowExW(
            WS_EX_CLIENTEDGE,
            class_name.as_ptr(),
            title.as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            x, y, w, h,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            h_instance,
            std::ptr::null_mut(),
        );

        create_converter_controls(hwnd, h_instance);
        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);
        *lock = Some(SafeHwnd(hwnd));
    }
}

unsafe fn create_converter_controls(parent: HWND, h_instance: HINSTANCE) {
    let font = create_bengali_font(16, 400);
    let font_btn = CreateFontW(14, 0, 0, 0, 600, 0, 0, 0, 1, 0, 0, 5, 0, to_wide_str("Segoe UI").as_ptr());

    // Top Edit Box (Avro / Unicode)
    let edit_avro = CreateWindowExW(
        WS_EX_CLIENTEDGE,
        to_wide_str("EDIT").as_ptr(),
        to_wide_str("আমার সোনার বাংলা, আমি তোমায় ভালোবাসি।").as_ptr(),
        WS_CHILD | WS_VISIBLE | WS_VSCROLL | ES_MULTILINE | ES_AUTOVSCROLL | ES_WANTRETURN,
        20, 35, 600, 130,
        parent,
        ID_EDIT_AVRO as HMENU,
        h_instance,
        std::ptr::null_mut(),
    );
    SendMessageW(edit_avro, WM_SETFONT, font as WPARAM, 1);

    // Convert Buttons (Middle row)
    let btn_to_bijoy = CreateWindowExW(
        0, to_wide_str("BUTTON").as_ptr(),
        to_wide_str("অভ্র থেকে ৩৬জুলাই (বিজয়) ➔").as_ptr(),
        WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
        20, 175, 190, 32,
        parent, ID_BTN_TO_BIJOY as HMENU, h_instance, std::ptr::null_mut(),
    );
    SendMessageW(btn_to_bijoy, WM_SETFONT, font_btn as WPARAM, 1);

    let btn_to_avro = CreateWindowExW(
        0, to_wide_str("BUTTON").as_ptr(),
        to_wide_str("➔ ৩৬জুলাই থেকে অভ্র").as_ptr(),
        WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
        220, 175, 170, 32,
        parent, ID_BTN_TO_AVRO as HMENU, h_instance, std::ptr::null_mut(),
    );
    SendMessageW(btn_to_avro, WM_SETFONT, font_btn as WPARAM, 1);

    let btn_swap = CreateWindowExW(
        0, to_wide_str("BUTTON").as_ptr(),
        to_wide_str("⇄ অদলবদল").as_ptr(),
        WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
        400, 175, 100, 32,
        parent, ID_BTN_SWAP as HMENU, h_instance, std::ptr::null_mut(),
    );
    SendMessageW(btn_swap, WM_SETFONT, font_btn as WPARAM, 1);

    let btn_clear = CreateWindowExW(
        0, to_wide_str("BUTTON").as_ptr(),
        to_wide_str("মুছুন").as_ptr(),
        WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
        495, 175, 55, 32,
        parent, ID_BTN_CLEAR as HMENU, h_instance, std::ptr::null_mut(),
    );
    SendMessageW(btn_clear, WM_SETFONT, font_btn as WPARAM, 1);

    let btn_copy = CreateWindowExW(
        0, to_wide_str("BUTTON").as_ptr(),
        to_wide_str("📋 কপি").as_ptr(),
        WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
        555, 175, 65, 32,
        parent, ID_BTN_COPY as HMENU, h_instance, std::ptr::null_mut(),
    );
    SendMessageW(btn_copy, WM_SETFONT, font_btn as WPARAM, 1);

    // Bottom Edit Box (36July / SutonnyMJ)
    let edit_bijoy = CreateWindowExW(
        WS_EX_CLIENTEDGE,
        to_wide_str("EDIT").as_ptr(),
        to_wide_str("Avgvi †mvbvi evsjv, Avwg †Zvgvq fv‡jvevwm|").as_ptr(),
        WS_CHILD | WS_VISIBLE | WS_VSCROLL | ES_MULTILINE | ES_AUTOVSCROLL | ES_WANTRETURN,
        20, 220, 600, 150,
        parent,
        ID_EDIT_BIJOY as HMENU,
        h_instance,
        std::ptr::null_mut(),
    );
    SendMessageW(edit_bijoy, WM_SETFONT, font as WPARAM, 1);
}

unsafe extern "system" fn converter_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_COMMAND => {
            let cmd = wparam & 0xFFFF;

            if cmd == ID_BTN_TO_BIJOY {
                let text = get_edit_text(hwnd, ID_EDIT_AVRO);
                let converted = avro_to_36july(&text);
                set_edit_text(hwnd, ID_EDIT_BIJOY, &converted);
            } else if cmd == ID_BTN_TO_AVRO {
                let text = get_edit_text(hwnd, ID_EDIT_BIJOY);
                let converted = bijoy_36july_to_avro(&text);
                set_edit_text(hwnd, ID_EDIT_AVRO, &converted);
            } else if cmd == ID_BTN_SWAP {
                let top = get_edit_text(hwnd, ID_EDIT_AVRO);
                let bot = get_edit_text(hwnd, ID_EDIT_BIJOY);
                set_edit_text(hwnd, ID_EDIT_AVRO, &bot);
                set_edit_text(hwnd, ID_EDIT_BIJOY, &top);
            } else if cmd == ID_BTN_CLEAR {
                set_edit_text(hwnd, ID_EDIT_AVRO, "");
                set_edit_text(hwnd, ID_EDIT_BIJOY, "");
            } else if cmd == ID_BTN_COPY {
                let bot = get_edit_text(hwnd, ID_EDIT_BIJOY);
                copy_to_clipboard(&bot);
            }
            0
        }
        WM_CLOSE => {
            ShowWindow(hwnd, SW_HIDE);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe fn get_edit_text(parent: HWND, id: usize) -> String {
    let h_edit = GetDlgItem(parent, id as i32);
    if h_edit.is_null() {
        return String::new();
    }
    let len = GetWindowTextLengthW(h_edit);
    if len <= 0 {
        return String::new();
    }
    let mut buf = vec![0u16; (len + 2) as usize];
    let text_len = GetWindowTextW(h_edit, buf.as_mut_ptr(), buf.len() as i32);
    from_wide_ptr(buf.as_ptr(), text_len as usize)
}

unsafe fn set_edit_text(parent: HWND, id: usize, text: &str) {
    let h_edit = GetDlgItem(parent, id as i32);
    if !h_edit.is_null() {
        let w = to_wide_str(text);
        SetWindowTextW(h_edit, w.as_ptr());
    }
}

unsafe fn copy_to_clipboard(text: &str) {
    let wide = to_wide_str(text);
    let bytes = (wide.len() * 2) as usize;
    const GMEM_MOVEABLE: u32 = 0x0002;
    const CF_UNICODETEXT: u32 = 13;
    let h_mem = GlobalAlloc(GMEM_MOVEABLE, bytes);
    if !h_mem.is_null() {
        let ptr = GlobalLock(h_mem) as *mut u16;
        if !ptr.is_null() {
            std::ptr::copy_nonoverlapping(wide.as_ptr(), ptr, wide.len());
            GlobalUnlock(h_mem);
            if OpenClipboard(std::ptr::null_mut()) != 0 {
                EmptyClipboard();
                SetClipboardData(CF_UNICODETEXT, h_mem);
                CloseClipboard();
            }
        }
    }
}

// ============================================================================
// 4. POPUP MENUS & HELP DIALOGS
// ============================================================================
unsafe fn show_layout_popup_menu(hwnd: HWND) {
    let h_menu = CreatePopupMenu();
    let current = bijoy_driver_win::get_current_mode();

    let f_eng = if current == BijoyMode::English { MF_CHECKED } else { MF_UNCHECKED };
    let f_uni = if current == BijoyMode::Unicode { MF_CHECKED } else { MF_UNCHECKED };
    let f_cla = if current == BijoyMode::Classic { MF_CHECKED } else { MF_UNCHECKED };
    let f_avr = if current == BijoyMode::Avro { MF_CHECKED } else { MF_UNCHECKED };

    AppendMenuW(h_menu, MF_STRING | f_eng, ID_MENU_ENGLISH, to_wide_str("🇬🇧 English (Ctrl+Alt+E)").as_ptr());
    AppendMenuW(h_menu, MF_STRING | f_uni, ID_MENU_UNICODE, to_wide_str("🇧🇩 July36 (Ctrl+Alt+V)").as_ptr());
    AppendMenuW(h_menu, MF_STRING | f_cla, ID_MENU_CLASSIC, to_wide_str("🔤 বিজয় ক্লাসিক - SutonnyMJ (Ctrl+Alt+B)").as_ptr());
    AppendMenuW(h_menu, MF_STRING | f_avr, ID_MENU_AVRO, to_wide_str("🗣️ অভ্র ফোনেটিক - Avro (F12)").as_ptr());

    let mut pt: POINT = std::mem::zeroed();
    GetCursorPos(&mut pt);
    SetForegroundWindow(hwnd);
    TrackPopupMenu(h_menu, TPM_LEFTALIGN | TPM_TOPALIGN, pt.x, pt.y, 0, hwnd, std::ptr::null());
    DestroyMenu(h_menu);
}

unsafe fn show_tray_popup_menu(hwnd: HWND) {
    let h_menu = CreatePopupMenu();

    AppendMenuW(h_menu, MF_STRING, ID_MENU_UNICODE, to_wide_str("🇧🇩 July36 (Ctrl+Alt+V)").as_ptr());
    AppendMenuW(h_menu, MF_STRING, ID_MENU_CLASSIC, to_wide_str("🔤 বিজয় ক্লাসিক (Ctrl+Alt+B)").as_ptr());
    AppendMenuW(h_menu, MF_STRING, ID_MENU_AVRO, to_wide_str("🗣️ অভ্র ফোনেটিক (F12)").as_ptr());
    AppendMenuW(h_menu, MF_STRING, ID_MENU_ENGLISH, to_wide_str("🇬🇧 English (Ctrl+Alt+E)").as_ptr());
    AppendMenuW(h_menu, MF_SEPARATOR, 0, std::ptr::null());
    AppendMenuW(h_menu, MF_STRING, ID_MENU_KEYBOARD, to_wide_str("⌨️ ভার্চুয়াল কীবোর্ড").as_ptr());
    AppendMenuW(h_menu, MF_STRING, ID_MENU_PREVIEW, to_wide_str("📝 প্রিভিউ উইন্ডো").as_ptr());
    AppendMenuW(h_menu, MF_STRING, ID_MENU_CONVERTER, to_wide_str("🌐 টেক্সট কনভার্টার").as_ptr());
    AppendMenuW(h_menu, MF_STRING, ID_MENU_HELP, to_wide_str("❓ টাইপিং নির্দেশিকা").as_ptr());
    AppendMenuW(h_menu, MF_SEPARATOR, 0, std::ptr::null());
    AppendMenuW(h_menu, MF_STRING, ID_MENU_EXIT, to_wide_str("❌ প্রস্থান (Exit)").as_ptr());

    let mut pt: POINT = std::mem::zeroed();
    GetCursorPos(&mut pt);
    SetForegroundWindow(hwnd);
    TrackPopupMenu(h_menu, TPM_LEFTALIGN | TPM_TOPALIGN, pt.x, pt.y, 0, hwnd, std::ptr::null());
    DestroyMenu(h_menu);
}

pub fn toggle_preview_window() {
    unsafe {
        let lock = PREVIEW_HWND.lock().unwrap();
        if let Some(h) = *lock {
            let hwnd = h.0;
            if IsWindowVisible(hwnd) != 0 {
                ShowWindow(hwnd, SW_HIDE);
            } else {
                ShowWindow(hwnd, SW_SHOWNA);
                SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW);
            }
        }
    }
}

pub fn show_virtual_keyboard() {
    unsafe {
        let mut lock = VK_HWND.lock().unwrap();
        if let Some(h) = *lock {
            let hwnd = h.0;
            if IsWindowVisible(hwnd) != 0 {
                ShowWindow(hwnd, SW_HIDE);
            } else {
                ShowWindow(hwnd, SW_SHOWNORMAL);
                SetForegroundWindow(hwnd);
            }
            return;
        }

        let h_instance = GetModuleHandleW(std::ptr::null());
        let screen_w = GetSystemMetrics(SM_CXSCREEN);
        let screen_h = GetSystemMetrics(SM_CYSCREEN);
        let w = 840;
        let h = 360;
        let x = (screen_w - w) / 2;
        let y = screen_h - h - 60;

        let class_name = to_wide_str("BijoyVirtualKeyboardClass");
        let title = to_wide_str("July36 কীবোর্ড লেআউট | National Standard BDS 1738:2018");

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            title.as_ptr(),
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_VISIBLE,
            x, y, w, h,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            h_instance,
            std::ptr::null_mut(),
        );

        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);
        *lock = Some(SafeHwnd(hwnd));
    }
}

unsafe extern "system" fn vk_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let hdc = BeginPaint(hwnd, &mut ps);
            let mut rc: RECT = std::mem::zeroed();
            GetClientRect(hwnd, &mut rc);

            let brush_bg = CreateSolidBrush(rgb(18, 24, 38));
            FillRect(hdc, &rc, brush_bg);
            DeleteObject(brush_bg as *mut c_void);

            SetBkMode(hdc, TRANSPARENT);

            let font_title = CreateFontW(16, 0, 0, 0, 700, 0, 0, 0, 1, 0, 0, 5, 0, to_wide_str("Segoe UI").as_ptr());
            let old_font = SelectObject(hdc, font_title as *mut c_void);
            SetTextColor(hdc, rgb(255, 255, 255));
            let mut t_rc = RECT { left: 16, top: 8, right: rc.right - 16, bottom: 30 };
            let t_w = to_wide_str("জাতীয় প্রমিত বাংলা কীবোর্ড লেআউট (National Standard BDS 1738:2018)");
            DrawTextW(hdc, t_w.as_ptr(), -1, &mut t_rc, DT_LEFT | DT_VCENTER | DT_SINGLELINE);
            SelectObject(hdc, old_font);
            DeleteObject(font_title as *mut c_void);

            let row1 = [
                ("`", "~", "~"), ("1", "১", "!"), ("2", "২", "@"), ("3", "৩", "#"),
                ("4", "৪", "$"), ("5", "৫", "%"), ("6", "৬", "^"), ("7", "৭", "&"),
                ("8", "৮", "*"), ("9", "৯", "("), ("0", "০", ")"), ("-", "-", "_"),
                ("=", "=", "+"),
            ];
            let row2 = [
                ("Q", "ঙ", "ঞ"), ("W", "য", "য়"), ("E", "ড", "ঢ"), ("R", "প", "ফ"),
                ("T", "ট", "ঠ"), ("Y", "চ", "ছ"), ("U", "জ", "ঝ"), ("I", "হ", "ঞ"),
                ("O", "গ", "ঘ"), ("P", "ড়", "ঢ়"), ("[", "[", "{"), ("]", "]", "}"),
            ];
            let row3 = [
                ("A", "র্", "া"), ("S", "ু", "ূ"), ("D", "ি", "ী"), ("F", "া", "অ"),
                ("G", "্", "লিংক"), ("H", "ব", "ভ"), ("J", "ক", "খ"), ("K", "ত", "থ"),
                ("L", "দ", "ধ"), (";", ";", ":"), ("'", "'", "\""),
            ];
            let row4 = [
                ("Z", "্য", "্র"), ("X", "ও", "ৌ"), ("C", "ে", "ৈ"), ("V", "র", "ল"),
                ("B", "ন", "ণ"), ("N", "স", "ষ"), ("M", "ম", "শ"), (",", ",", "<"),
                (".", ".", ">"), ("/", "/", "?"),
            ];

            let font_eng = CreateFontW(11, 0, 0, 0, 600, 0, 0, 0, 1, 0, 0, 5, 0, to_wide_str("Segoe UI").as_ptr());
            let font_bn = create_bengali_font(16, 700);
            let brush_key = CreateSolidBrush(rgb(30, 41, 59));
            let brush_border = CreateSolidBrush(rgb(71, 85, 105));

            let draw_row = |hdc: HDC, keys: &[(&str, &str, &str)], start_x: i32, start_y: i32, key_w: i32| {
                for (i, &(eng, norm, shft)) in keys.iter().enumerate() {
                    let kx = start_x + (i as i32 * (key_w + 6));
                    let ky = start_y;
                    let k_rc = RECT { left: kx, top: ky, right: kx + key_w, bottom: ky + 46 };

                    let old_b = SelectObject(hdc, brush_border as *mut c_void);
                    RoundRect(hdc, k_rc.left, k_rc.top, k_rc.right, k_rc.bottom, 6, 6);
                    SelectObject(hdc, old_b);

                    let inner_rc = RECT { left: kx + 1, top: ky + 1, right: kx + key_w - 1, bottom: ky + 45 };
                    let old_b2 = SelectObject(hdc, brush_key as *mut c_void);
                    RoundRect(hdc, inner_rc.left, inner_rc.top, inner_rc.right, inner_rc.bottom, 4, 4);
                    SelectObject(hdc, old_b2);

                    let old_f = SelectObject(hdc, font_eng as *mut c_void);
                    SetTextColor(hdc, rgb(148, 163, 184));
                    let mut e_rc = RECT { left: kx + 4, top: ky + 2, right: kx + 22, bottom: ky + 16 };
                    let e_w = to_wide_str(eng);
                    DrawTextW(hdc, e_w.as_ptr(), -1, &mut e_rc, DT_LEFT | DT_TOP | DT_SINGLELINE);

                    SelectObject(hdc, font_bn as *mut c_void);
                    SetTextColor(hdc, rgb(255, 255, 255));
                    let mut n_rc = RECT { left: kx + 3, top: ky + 16, right: kx + key_w / 2 + 5, bottom: ky + 43 };
                    let n_w = to_wide_str(norm);
                    DrawTextW(hdc, n_w.as_ptr(), -1, &mut n_rc, DT_CENTER | DT_VCENTER | DT_SINGLELINE);

                    SetTextColor(hdc, rgb(251, 146, 60));
                    let mut s_rc = RECT { left: kx + key_w / 2, top: ky + 16, right: kx + key_w - 2, bottom: ky + 43 };
                    let s_w = to_wide_str(shft);
                    DrawTextW(hdc, s_w.as_ptr(), -1, &mut s_rc, DT_CENTER | DT_VCENTER | DT_SINGLELINE);

                    SelectObject(hdc, old_f);
                }
            };

            draw_row(hdc, &row1, 16, 36, 54);
            draw_row(hdc, &row2, 36, 88, 54);
            draw_row(hdc, &row3, 52, 140, 54);
            draw_row(hdc, &row4, 68, 192, 54);

            let space_rc = RECT { left: 220, top: 246, right: 580, bottom: 278 };
            let old_b = SelectObject(hdc, brush_border as *mut c_void);
            RoundRect(hdc, space_rc.left, space_rc.top, space_rc.right, space_rc.bottom, 6, 6);
            SelectObject(hdc, old_b);
            let mut sp_text_rc = space_rc;
            let old_f = SelectObject(hdc, font_eng as *mut c_void);
            SetTextColor(hdc, rgb(203, 213, 225));
            let sp_text = to_wide_str("Space (স্পেস)");
            DrawTextW(hdc, sp_text.as_ptr(), -1, &mut sp_text_rc, DT_CENTER | DT_VCENTER | DT_SINGLELINE);
            SelectObject(hdc, old_f);

            let font_legend = CreateFontW(12, 0, 0, 0, 400, 0, 0, 0, 1, 0, 0, 5, 0, to_wide_str("Segoe UI").as_ptr());
            SelectObject(hdc, font_legend as *mut c_void);
            SetTextColor(hdc, rgb(156, 163, 175));
            let mut leg_rc = RECT { left: 16, top: 288, right: rc.right - 16, bottom: 312 };
            let leg_text = to_wide_str("সাদা: সাধারণ অক্ষর  |  কমলা: শিফট+অক্ষর  |  যুক্তাক্ষর: অক্ষর + g + অক্ষর  |  হটকি: F12 (টগল), Ctrl+Alt+V (বিজয়)");
            DrawTextW(hdc, leg_text.as_ptr(), -1, &mut leg_rc, DT_CENTER | DT_VCENTER | DT_SINGLELINE);

            DeleteObject(brush_key as *mut c_void);
            DeleteObject(brush_border as *mut c_void);
            DeleteObject(font_eng as *mut c_void);
            DeleteObject(font_bn as *mut c_void);
            DeleteObject(font_legend as *mut c_void);

            EndPaint(hwnd, &ps);
            0
        }
        WM_CLOSE => {
            ShowWindow(hwnd, SW_HIDE);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe fn show_help_dialog(parent: HWND) {
    let msg = to_wide_str(
        "July36 বাংলা কীবোর্ড নির্দেশিকা (BDS 1738)\n\n\
        • স্বরবর্ণ: F=অ, gf=আ, gd=ই, gD=ঈ, gs=উ, ga=ঋ, gc=এ, gC=ঐ, x=ও, gX=ঔ\n\
        • প্রি-কার: dk=তি, ck=তে, ckf=তো, ckX=তৌ\n\
        • যুক্তাক্ষর (g দিয়ে): jgk=ক্ত, jgN=ক্ষ, ugI=জ্ঞ, igm=হ্ম, Igd=ঞ্চি\n\
        • রেফ ও ফলা: Aj=র্ক, jz=ক্র, jZ=ক্য\n\
        • মুদ্রা ও বিশেষ: g1..g8=আনা/গণ্ডা, g,=ৰ, g.=ৱ, g#=ঽ, gG=॥\n\n\
        হটকি: F12 (টগল), Ctrl+Alt+V (July36), Ctrl+Alt+B (ক্লাসিক), Ctrl+Alt+E (ইংরেজি)",
    );
    let title = to_wide_str("📖 July36 টাইপিং নির্দেশিকা");
    MessageBoxW(parent, msg.as_ptr(), title.as_ptr(), 0x00000040);
}

unsafe fn show_settings_dialog(parent: HWND) {
    let msg = to_wide_str(
        "July36 কীবোর্ড সেটিংস:\n\n\
        ✓ সিস্টেম-ওয়াইড লো-লেভেল কীবোর্ড হুক সক্রিয় আছে\n\
        ✓ উইন্ডোজ স্টার্টআপ অটো-রান চালু আছে\n\
        ✓ প্রিভিউ উইন্ডো রিয়েল-টাইম প্রেডিকশন চালু আছে\n\n\
        হটকি: Ctrl+Alt+V (July36), Ctrl+Alt+B (ক্লাসিক), F12 (অভ্র), Ctrl+Alt+E (ইংরেজি)",
    );
    let title = to_wide_str("⚙️ July36 সেটিংস");
    MessageBoxW(parent, msg.as_ptr(), title.as_ptr(), 0x00000040);
}

unsafe fn show_about_dialog(parent: HWND) {
    let msg = to_wide_str(
        "July36 বাংলা কীবোর্ড (July36 Keyboard)\n\
        সংস্করণ: ১.০.০ (Version 1.0.0 Native Windows)\n\
        জাতীয় মান: BDS 1738:2018 / 1935:2018 / 1520:2018\n\n\
        উইন্ডোজের জন্য সম্পূর্ণ নেটিভ ডেস্কটপ কীবোর্ড সফটওয়্যার।",
    );
    let title = to_wide_str("ℹ️ July36 পরিচিতি");
    MessageBoxW(parent, msg.as_ptr(), title.as_ptr(), 0x00000040);
}

// ============================================================================
// 5. SYSTEM TRAY ICON
// ============================================================================
unsafe fn setup_tray_icon(hwnd: HWND, _h_instance: HINSTANCE) {
    let mut nid: NOTIFYICONDATAW = std::mem::zeroed();
    nid.cb_size = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
    nid.hwnd = hwnd;
    nid.u_id = TRAY_ICON_ID;
    nid.u_flags = NIF_MESSAGE | NIF_TIP;
    nid.u_callback_message = WM_TRAYICON;

    let tip = to_wide_str("July36 বাংলা কীবোর্ড");
    let copy_len = tip.len().min(127);
    nid.sz_tip[..copy_len].copy_from_slice(&tip[..copy_len]);

    Shell_NotifyIconW(NIM_ADD, &nid);
}

unsafe fn remove_tray_icon(hwnd: HWND) {
    let mut nid: NOTIFYICONDATAW = std::mem::zeroed();
    nid.cb_size = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
    nid.hwnd = hwnd;
    nid.u_id = TRAY_ICON_ID;
    Shell_NotifyIconW(NIM_DELETE, &nid);
}
