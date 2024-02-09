use std::mem::size_of;
use std::time::Duration;

use komorebi_core::client::SendMessage;
use komorebi_core::SocketMessage;
use komorebi_core::state::{State, Window};
use i_slint_backend_winit::WinitWindowAccessor;
use slint::{run_event_loop, TimerMode};
use winit::dpi::{PhysicalPosition, Position};
use winit::platform::windows::{WindowBuilderExtWindows, WindowExtWindows, HWND } ;
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle, Win32WindowHandle};
use windows::Win32::UI::Shell::APPBARDATA;
use windows::Win32::UI::Shell::SHAppBarMessage;
use windows::Win32::Foundation::HWND as Win32HWND;
use winit::window::WindowLevel;

#[repr(u32)]
enum AppBarMessage {
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
struct AppBar(APPBARDATA);
impl AppBar {
    fn new(hwnd: HWND) -> Self {
        let mut abd = APPBARDATA{
            cbSize: size_of::<APPBARDATA> as u32,
            hWnd: Win32HWND(hwnd.into()),
            uEdge: 1,
            ..APPBARDATA::default()
        };
        let ret = unsafe {
            SHAppBarMessage(AppBarMessage::New as u32, &mut abd as *mut APPBARDATA)
        };
        println!("{:#?}", ret);
        Self(abd)
    }
    fn set_pos(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.0.rc.left = x;
        self.0.rc.top = y;
        self.0.rc.right = x + width;
        self.0.rc.bottom = y + height;
        println!("{:#?}", self.0.rc);
        let ret = unsafe {
            SHAppBarMessage(AppBarMessage::SetPos as u32, &mut self.0 as *mut APPBARDATA)
        };
        println!("{:#?} {}", self.0.rc, ret);
    }
    fn window_query_pos(&mut self) {
    let ret=    unsafe {
            SHAppBarMessage(AppBarMessage::QueryPos as u32, &mut self.0 as *mut APPBARDATA)
        };
        println!("{:#?}, {}", self.0.rc, ret);
    }
    fn activate(&mut self) {
        let ret = unsafe {
            SHAppBarMessage(AppBarMessage::Activate as u32, &mut self.0 as *mut APPBARDATA)
        };
        println!("{:#?}", ret);
    }
}
fn main() {
    let mut be = i_slint_backend_winit::Backend::new().unwrap();
    be.window_builder_hook = Some(Box::new(|wb| {
        println!("windows builder hook");
        wb.with_position(PhysicalPosition::new(0, 0))
        .with_decorations(false)
        
    }));
    slint::platform::set_platform(Box::new(be)).unwrap();

    // let state: State = SocketMessage::State.send_receive().unwrap();
    // println!("{:#?}", state);
    let w = MainWindow::new().unwrap();
    w.show().unwrap();
    //sleep 10
    // set a timer callback in slint
    let t = slint::Timer::default();
    // t.start(TimerMode::SingleShot, std::time::Duration::from_millis(1000), move || {
        w.window().with_winit_window(|winit_window: &winit::window::Window| {
            winit_window.set_decorations(false);
            // get monitor width
            let monitor = winit_window.current_monitor().unwrap();
            let monitor_size = monitor.size();
            winit_window.set_outer_position(Position::Physical(PhysicalPosition::new(0, 0)));
            winit_window.set_min_inner_size(Some(winit::dpi::PhysicalSize::new(monitor_size.width, 10)));
            winit_window.set_max_inner_size(Some(winit::dpi::PhysicalSize::new(monitor_size.width, 10)));
            winit_window.request_inner_size(winit::dpi::PhysicalSize::new(monitor_size.width, 10));
            winit_window.set_window_level(WindowLevel::AlwaysOnTop);

            let wid = winit_window.window_handle().unwrap().as_raw();
            if let RawWindowHandle::Win32(wid) = wid {
                let mut abp = AppBar::new(wid.hwnd.into());
                abp.window_query_pos();
                abp.set_pos(0, 0, monitor_size.width as i32, 10);
                abp.window_query_pos();
                abp.activate();
    
            }
        });
    //  });
    // std::thread::sleep(std::time::Duration::from_secs(10));
    run_event_loop().unwrap();
}

slint::slint! {

    export component MainWindow inherits Window {
        no_frame: true;
        always_on_top: true;
        Text {
            text: "hello world";
            color: green;
        }
    }
}