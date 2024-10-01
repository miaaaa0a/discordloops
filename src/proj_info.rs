use windows::core::{Error, HSTRING};
use windows::Win32::UI::WindowsAndMessaging::{EnumChildWindows, FindWindowExW, FindWindowW, GetWindowTextW, GetClassNameW};
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};

pub fn get_fl() -> Result<HWND, Error> {
    let fl_class = &HSTRING::from("TFruityLoopsMainForm");
    unsafe {
        return FindWindowW(fl_class, None);
    }
}

pub fn get_fl_title(hwnd: HWND) -> String {
    let mut buf: [u16; 512] = [0; 512];
    let title: String;
    unsafe {
        let strlen = GetWindowTextW(hwnd, &mut buf);
        title = String::from_utf16_lossy(&buf[..strlen as usize]);
    }
    return title;
}

fn get_plugins(phwnd: HWND) -> Vec<HWND> {
    let cbar_class = &HSTRING::from("TQuickControlBar");
    let wpc_class = &HSTRING::from("TWPControl");
    let mut plugins: Vec<HWND> = vec![];

    extern "system" fn enum_child_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        unsafe {
            let plg = lparam.0 as *mut Vec<HWND>;
            let mut buf: [u16; 512] = [0; 512];
            let len = GetClassNameW(hwnd, &mut buf);
            let class = String::from_utf16_lossy(&buf[..len as usize]);

            if class == "TPluginForm" {
                plg.as_mut()
                .expect("couldnt reference plugin array")
                .push(hwnd);
            }
            return true.into();
        }
    }
    
    unsafe {
        let cbar = FindWindowExW(phwnd, None, cbar_class, None).unwrap_or_default();
        let wpc = FindWindowExW(phwnd, cbar, wpc_class, None).unwrap_or_default();
        let _ = EnumChildWindows(wpc, Some(enum_child_proc), LPARAM(&mut plugins as *mut Vec<HWND> as isize));
    }

    return plugins;
}

pub fn count_ott(result: &Result<HWND, Error>) -> String {
    let fl_hwnd: HWND;
    match result {
        Ok(h) => fl_hwnd = *h,
        Err(e) => {
            println!("error! {}", e);
            return String::from("not using otts right now");
        },
    }
    let mut buf: [u16; 512] = [0; 512];
    let mut otts: u8 = 0;
    let hwnds = get_plugins(fl_hwnd);
    
    unsafe {
        for i in hwnds {
            let strlen = GetWindowTextW(i, &mut buf);
            let title = String::from_utf16_lossy(&buf[..strlen as usize]);
            if title.starts_with("OTT") {
                otts += 1;
            }
        }
    }

    let mut formatted = otts.to_string();
    let mut ott_ref = String::from(" otts");
    if otts == 1 {
        ott_ref = String::from(" ott");
    }

    formatted.insert_str(0, "using ");
    formatted.push_str(&ott_ref);
    return formatted;
}

pub fn get_project(result: &Result<HWND, Error>) -> String {
    let hwnd: HWND;
    match result {
        Ok(h) => hwnd = *h,
        Err(e) => {
            println!("error! {}", e);
            return String::from("nothing here...");
        },
    }

    let mut fl_title = get_fl_title(hwnd);

    if fl_title != "nothing here..." {
        fl_title.truncate(fl_title.len().saturating_sub(17));
        println!("project: {fl_title}");
        fl_title.insert_str(0, "working on ");
    }

    return fl_title;
}