use std::ffi::OsStr;
use std::io::Error;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;

// TODO use windows crate instread of winapi

use winapi::shared::minwindef::{
    ATOM, BOOL, BYTE, DWORD, HINSTANCE, HIWORD, HKL, HMODULE, HRGN, HWINSTA, INT, LOWORD, LPARAM,
    LPBYTE, LPDWORD, LPINT, LPVOID, LPWORD, LRESULT, PBYTE, PUINT, PULONG, TRUE, UCHAR, UINT,
    ULONG, USHORT, WORD, WPARAM,
};
use winapi::shared::windef::{
    COLORREF, DPI_AWARENESS, DPI_AWARENESS_CONTEXT, DPI_HOSTING_BEHAVIOR, HACCEL, HBITMAP, HBRUSH,
    HCURSOR, HDC, HDESK, HHOOK, HICON, HMENU, HMONITOR, HWINEVENTHOOK, HWND, LPCRECT, LPPOINT,
    LPRECT, POINT, RECT, SIZE,
};
use winapi::um::minwinbase::LPSECURITY_ATTRIBUTES;
use winapi::um::processthreadsapi::GetCurrentThreadId;
use winapi::um::wingdi::{
    BLENDFUNCTION, DEVMODEA, DEVMODEW, LOGFONTA, LOGFONTW, PDISPLAY_DEVICEA, PDISPLAY_DEVICEW,
};
use winapi::um::winnt::{
    ACCESS_MASK, BOOLEAN, CHAR, HANDLE, LONG, LPCSTR, LPCWSTR, LPSTR, LPWSTR, LUID,
    PSECURITY_DESCRIPTOR, PSECURITY_INFORMATION, PVOID, SHORT, VOID, WCHAR,
};

use __core::mem::MaybeUninit;
use imgui::*;
use inputbot::MouseCursor;
use winapi::um::winuser::{
    EnumDesktopWindows, FindWindowW, GetCursorPos, GetDesktopWindow, GetForegroundWindow,
    GetShellWindow, GetThreadDesktop, GetTopWindow, GetWindowRect, SPIF_UPDATEINIFILE,
    SPI_GETMOUSEKEYS,
};
use winapi::um::winuser::{
    SystemParametersInfoA, SystemParametersInfoW, SPIF_SENDCHANGE, SPI_GETWORKAREA,
    SPI_SETDESKWALLPAPER,
};

use crate::Resolution;
use crate::{game_interactions, shared::BotMode, statics};

pub fn window_size() -> (RECT, i32, i32) {
    let mut rect: MaybeUninit<RECT> = MaybeUninit::uninit();
    unsafe {
        use core::ptr;
        // let mut hwnd: HWND = ptr::null_mut();
        // let id = GetCurrentThreadId();
        // let desk = GetThreadDesktop(id);
        // println!("desk {:?} ", desk);

        let title: Vec<u16> = OsStr::new("Gloria Victis")
            .encode_wide()
            .chain(once(0))
            .collect();
        let hwnd = FindWindowW(ptr::null(), title.as_ptr());
        // println!("hwnd {:?} ", hwnd);

        let forground = GetForegroundWindow();

        // println!(
        //     "forground {:?} DesktopWindow {:?} ShellWindow {:?}",
        //     forground,
        //     GetDesktopWindow(),
        //     GetShellWindow()
        // );
        if !forground.eq(&GetDesktopWindow()) && !forground.eq(&GetShellWindow()) {
            let res = GetWindowRect(hwnd, rect.as_mut_ptr());
            if res == TRUE {
                // println!("res {}", res);
            } else {
                // println!("{}", Error::last_os_error());
            }
            let rect = rect.assume_init();
            // println!("rect {:?} {:?} {:?} {:?}", rect.left, rect.top, rect.right, rect.bottom);
            if rect.right > rect.left && rect.bottom > rect.top {
                let width = rect.right - rect.left;
                let height = rect.bottom - rect.top;
                (rect, width, height)
            } else {
                screen()
            }
        } else {
            screen()
        }
    }
}

winapi::STRUCT! {
    struct MOUSEKEYS {
    cbSize: UINT,
    dwFlags: DWORD,
    iMaxSpeed: DWORD,
    iTimeToMaxSpeed: DWORD,
    iCtrlSpeed: DWORD,
    dwReserved1: DWORD,
    dwReserved2: DWORD,
  }
}

pub fn buttons() -> (i32, i32) {
    let mut rect: MaybeUninit<MOUSEKEYS> = MaybeUninit::uninit();
    let rect = unsafe {
        let res = SystemParametersInfoA(SPI_GETMOUSEKEYS, 0, rect.as_mut_ptr() as PVOID, 0);

        if res == TRUE {
            // println!("res {}", res);
        } else {
            println!("{}", Error::last_os_error());
        }
        rect.assume_init()
    };
    print!(
        "buttons {:?} {:?} {:?} {:?}",
        rect.cbSize, rect.dwFlags, rect.dwReserved1, rect.dwReserved2
    );
    (0, 0)
}

pub fn pos() -> (i32, i32) {
    unsafe {
        let mut point = MaybeUninit::uninit();
        GetCursorPos(point.as_mut_ptr());
        let point = point.assume_init();
        (point.x, point.y)
    }
}
pub fn screen() -> (RECT, i32, i32) {
    let mut rect: MaybeUninit<RECT> = MaybeUninit::uninit();
    let rect = unsafe {
        let res = SystemParametersInfoW(SPI_GETWORKAREA, 0, rect.as_mut_ptr() as PVOID, 0);
        if res == TRUE {
            // println!("res {}", res);
        } else {
            println!("{}", Error::last_os_error());
        }
        rect.assume_init()
    };
    // print!("rect {:?} {:?} {:?} {:?}", rect.left, rect.top, rect.right, rect.bottom);
    (rect, rect.right, rect.bottom)
}

// fn wnd_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
//     match msg {
//         WM_DESTROY => {
//             PostQuitMessage(0); 0
//         },
//         WM_PAINT => {
//             let mut ps = zeroed();
//             let hdc = BeginPaint(hwnd, &mut ps);
//             TextOutA(hdc, 5, 5,
//                 SZ_TEXT.as_ptr() as *const i8,
//                 SZ_TEXT.len() as i32
//             );
//             0
//         },
//         _ => {
//             DefWindowProcW(hwnd, msg, wparam, lparam)
//         }
//     }
// }
