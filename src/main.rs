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
use winit::window::WindowLevel;
mod winappbar;
use winappbar::AppBar;
use winappbar::AppBarMessage;
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
    t.start(TimerMode::SingleShot, std::time::Duration::from_millis(100), move || {
        w.window().with_winit_window(|winit_window: &winit::window::Window| {
            winit_window.set_decorations(false);
            // get monitor width
            let monitor = winit_window.current_monitor().unwrap();
            let height = 10;
            let monitor_size = monitor.size();
            winit_window.set_outer_position(Position::Physical(PhysicalPosition::new(0, 0)));
            winit_window.set_min_inner_size(Some(winit::dpi::PhysicalSize::new(monitor_size.width, height)));
            winit_window.set_max_inner_size(Some(winit::dpi::PhysicalSize::new(monitor_size.width, height)));
            winit_window.request_inner_size(winit::dpi::PhysicalSize::new(monitor_size.width, height));
            winit_window.set_window_level(WindowLevel::AlwaysOnTop);

            let wid = winit_window.window_handle().unwrap().as_raw();
            if let RawWindowHandle::Win32(wid) = wid {
                let apb = AppBar::new(wid.hwnd.into());
                apb.window_query_pos();
                apb.set_pos(0, 0, monitor_size.width, height);
                apb.window_query_pos();
                apb.set_pos(0, 0, monitor_size.width, height);
                apb.window_query_pos();
                // w.window().on_close_requested(move || {
                //     apb.remove();
                //     slint::CloseRequestResponse::HideWindow
                // });
                ctrlc::set_handler(move || {
                    println!("received Ctrl+C!");
                    apb.remove();
                    std::process::exit(0);
                }).unwrap();
        
            }
            winit_window.set_outer_position(Position::Physical(PhysicalPosition::new(0, 0)));
         
        });
     });
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