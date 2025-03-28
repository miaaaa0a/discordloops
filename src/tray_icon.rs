use anyhow::Error;
use std::ffi::{c_void, OsString};
use std::os::windows::ffi::OsStrExt;
use windows::core::{w, BOOL, HSTRING, PWSTR};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows::Win32::Graphics::Gdi::ValidateRect;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Shell::{
    Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NOTIFYICONDATAW,
};
use windows::Win32::UI::WindowsAndMessaging::*;

const WM_TRAYMESSAGE: u32 = WM_USER + 0x100;
const TRAY_ID: u32 = 3030303;

pub struct WindowCreatedData {
    pub hwnd: HWND,
}

unsafe impl std::marker::Send for WindowCreatedData {}

pub enum WindowMessages {
    MouseRightButtonDown,
    MouseRightButtonUp,
    WindowCreated(WindowCreatedData),
    WindowClosed,
}

pub struct Window {
    //    message_receiver: std::sync::mpsc::Receiver<WindowMessages>,
    pub hwnd: HWND,
}

pub struct WindowThreadState {
    pub message_sender: std::sync::mpsc::Sender<WindowMessages>,
    pub is_tracking: bool,
}

#[allow(clippy::result_unit_err)]
pub fn create_window() -> Result<Window, ()> {
    let (channel_sender, channel_receiver) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let mut window_state = WindowThreadState {
            message_sender: channel_sender,
            is_tracking: false,
        };

        unsafe {
            let instance = GetModuleHandleW(None).unwrap();
            let class_name = w!("DiscordLoops Tray Window Class");

            let wclass = WNDCLASSW {
                hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
                hInstance: instance.into(),
                lpszClassName: class_name,
                style: CS_NOCLOSE,
                lpfnWndProc: Some(wndproc),
                ..Default::default()
            };

            let atom = RegisterClassW(&wclass);
            debug_assert!(atom != 0);

            let _hwnd = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                class_name,
                w!("DiscordLoops Tray Window"),
                WS_DISABLED | WS_MINIMIZE,
                0,
                0,
                1,
                1,
                None,
                None,
                None,
                Some(&mut window_state as *mut WindowThreadState as *mut c_void),
            )
            .unwrap();
            println!("hwnd in func: {:?}", _hwnd);

            let mut message = MSG::default();
            while GetMessageW(&mut message, None, 0, 0).into() {
                DispatchMessageW(&message);
                let _ = TranslateMessage(&message);
            }
        }
    });

    if let WindowMessages::WindowCreated(x) = channel_receiver.recv().unwrap() {
        return Ok(Window {
            //message_receiver: channel_receiver,
            hwnd: x.hwnd,
        });
    }

    Err(())
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_CREATE => {
                // retrieve the message struct that contains the creation parameters
                let create_struct = lparam.0 as *const CREATESTRUCTW;

                // retrieve the rust window state
                let window_state_ptr =
                    create_struct.as_ref().unwrap().lpCreateParams as *mut WindowThreadState;
                let window_state: &mut WindowThreadState = window_state_ptr.as_mut().unwrap();

                // the state we can store inside the user data parameter of the window
                SetWindowLongPtrW(window, GWLP_USERDATA, window_state_ptr as isize);

                window_state
                    .message_sender
                    .send(WindowMessages::WindowCreated(WindowCreatedData {
                        hwnd: window,
                    }))
                    .unwrap();
                LRESULT(0)
            }
            WM_PAINT => {
                println!("WM_PAINT");
                _ = ValidateRect(Some(window), None);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_TRAYMESSAGE => {
                //println!("fuckkckc");
                match lparam.0 as u32 {
                    WM_RBUTTONDOWN => {
                        println!("RMB down");
                        let mut clickpoint = POINT::default();
                        let _ = GetCursorPos(&mut clickpoint as *mut POINT);
                        let pop_menu = CreatePopupMenu().unwrap();
                        
                        let exit_item = MENUITEMINFOW {
                            cbSize: std::mem::size_of::<MENUITEMINFOW>() as u32,
                            fMask: MIIM_STRING | MIIM_ID,
                            wID: 1,
                            dwTypeData: PWSTR(w!("Exit").as_ptr() as *mut _),
                            ..Default::default()
                        };
                        let _ = InsertMenuItemW(pop_menu, u32::MAX, true, &exit_item);
                        let _ = SetForegroundWindow(window);
                        let selected_item = TrackPopupMenu(
                            pop_menu,
                            TPM_LEFTALIGN | TPM_LEFTBUTTON | TPM_BOTTOMALIGN,
                            clickpoint.x,
                            clickpoint.y,
                            Some(0),
                            window,
                            None,
                        );

                        // i might add more stuff in the future so im allowing a single match
                        #[allow(clippy::single_match)]
                        match selected_item.0 {
                            1 => {
                                let _ = PostMessageW(Some(window), WM_CLOSE, WPARAM(0), LPARAM(0));
                                std::process::exit(0);
                            }
                            _ => {}
                        }
                    }
                    _ => tracing::debug!("Unhandled tray message: 0x{:x}", lparam.0 as u32),
                }
                LRESULT(0)
            }
            _ => {
                tracing::debug!("Unhandled message: 0x{:x}", message);
                DefWindowProcW(window, message, wparam, lparam)
            }
        }
    }
}

pub fn draw_tray_icon(phwnd: HWND) -> Result<BOOL, Error> {
    let mut tip = [0u16; 128];

    let tip_bytes: Vec<u16> = OsString::from("DiscordLoops")
        .as_os_str()
        .encode_wide()
        .take(128)
        .collect();

    unsafe {
        std::ptr::copy_nonoverlapping(
            tip_bytes.as_ptr(),
            tip.as_mut_ptr(),
            // Ensure we don't read past the end of info_bytes, or
            // copy too much memory.
            tip_bytes.len().min(128),
        );
    }

    unsafe {
        let icon = LoadImageW(
            None,
            &HSTRING::from("icon.ico"),
            IMAGE_ICON,
            256,
            256,
            LR_LOADFROMFILE,
        )?;
        let nid = NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
            uFlags: NIF_TIP | NIF_ICON | NIF_MESSAGE,
            szTip: tip,
            hIcon: HICON(icon.0),
            hWnd: phwnd,
            uCallbackMessage: WM_TRAYMESSAGE,
            uID: TRAY_ID,
            ..Default::default()
        };

        Ok(Shell_NotifyIconW(NIM_ADD, &nid))
    }
}
