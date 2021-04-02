//! Graphical user interface creation and management utilities.
mod config;
mod error;
mod window;
pub use error::Error;
use window::WINDOW;

/// Draw a GUI window.
pub fn createWindow() {
    WINDOW.mainloop();
}

// lazy_static! {
//     /// The current active window
//     static ref WINDOW: Mutex<Option<Window>> = Mutex::default();
// }

// /// Create a new GUI window.
// ///
// /// This will cause an error if an active window already exists.
// /// Note that this does not draw the window, [drawWindow](drawWindow) must be called seperately.
// pub fn createWindow() -> Result<(), Error> {
//     let mut win = WINDOW.lock().unwrap();
//     match *win {
//         Some(_) => Err(Error::WindowExists),
//         None => {
//             // Init a new window
//             let _ = mem::replace(&mut *win, Some(Window::create()));
//             Ok(())
//         }
//     }
// }

// /// Draw the current window.
// ///
// /// This will cause an error if an active window does not exist.
// pub fn drawWindow() -> Result<(), Error> {
//     let mut win = WINDOW.lock().unwrap();
//     match &mut *win {
//         Some(win) => {
//             win.draw();
//             Ok(())
//         }
//         None => Err(Error::WindowNotFound),
//     }
// }
