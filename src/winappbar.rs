use std::sync::Mutex;

use windows::Win32::UI::Shell::APPBARDATA;
use windows::Win32::UI::Shell::SHAppBarMessage;
use windows::Win32::Foundation::HWND;


#[repr(u32)]
pub(crate) enum AppBarMessage {
    New = 0,
    Remove = 1,
    QueryPos = 2,
    SetPos = 3,
    GetState = 4,
    GetTaskbarPos = 5,
    Activate = 6,
    GetAutoHideBar = 7,
    SetAutoHideBar = 8,
    WindowPosChanged = 9,
    SetState = 10,
    GetAutoHideBarEx = 11,
    SetAutoHideBarEx = 12,
}
pub(crate) struct AppBar {
    d: Mutex<APPBARDATA>,
}

impl AppBar {
    pub(crate)fn new(hwnd: isize) -> Self {
        let mut abd = APPBARDATA{
            cbSize: 48,
            hWnd: HWND(hwnd),
            uEdge: 1,
            ..APPBARDATA::default()
        };
        unsafe {
            SHAppBarMessage(AppBarMessage::New as u32, &mut abd as *mut APPBARDATA)
        };
        Self{d: Mutex::from(abd)}
    }
    fn send(&self, msg: AppBarMessage) {
        unsafe {
            SHAppBarMessage(msg as u32, &mut *self.d.lock().unwrap() as *mut APPBARDATA)
        };
    }
    pub(crate)fn set_pos(&self, x: u32, y: u32, width: u32, height: u32) {
        {
            let mut apb: std::sync::MutexGuard<'_, APPBARDATA> = self.d.lock().unwrap();
            apb.rc.left = x as i32;
            apb.rc.top = y as i32;
            apb.rc.right = (x + width) as i32;
            apb.rc.bottom = (y + height) as i32;
        }
        self.send(AppBarMessage::SetPos);
    }
    pub(crate)fn window_query_pos(&self) {
        self.send(AppBarMessage::QueryPos);
        let d = self.d.lock().unwrap();
        println!("{} {} {} {}", d.rc.left, d.rc.top, d.rc.right, d.rc.bottom);
    }
    pub(crate) fn remove(&self) {
        self.send(AppBarMessage::Remove);
    }
}