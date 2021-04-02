use super::config::*;
use iui::controls;
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};
use std::thread;

lazy_static! {
    pub static ref WINDOW: Window = Window::new();
}

#[derive(Clone)]
struct UI(Arc<RwLock<iui::UI>>);
unsafe impl Send for UI {}
unsafe impl Send for Win {}

#[derive(Clone)]
struct Win(Arc<RwLock<controls::Window>>);
unsafe impl Sync for UI {}
unsafe impl Sync for Win {}

pub struct Window {
    ui: UI,
    win: Win,
}

impl Window {
    fn new() -> Self {
        let ui = iui::UI::init().unwrap();
        let win = controls::Window::new(
            &ui,
            *Name::default(),
            *Height::default(),
            *Width::default(),
            *Menu::default(),
        );

        Window {
            ui: UI(Arc::new(RwLock::new(ui))),
            win: Win(Arc::new(RwLock::new(win))),
        }
    }

    pub fn mainloop(&self) {
        let ui = self.ui.clone();
        let win = self.win.clone();

        // Start the mainloop in a background thread to prevent blocking
        thread::spawn(move || {
            win.0.write().unwrap().show(&ui.0.read().unwrap());
            ui.0.write().unwrap().main();
        });
    }
}
