#[macro_use]
extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;
extern crate gepub_sys;
extern crate gtk;

macro_rules! assert_initialized_main_thread {
    () => {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    };
}

mod auto;
mod doc;
mod nav_point;

pub use auto::*;
pub use doc::DocExtManual;
pub use nav_point::NavPoint;
