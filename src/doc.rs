use glib::translate::{FromGlibPtrContainer, ToGlibPtr};

use Doc;
use NavPoint;

pub trait DocExtManual {
    fn get_toc(&self) -> Vec<NavPoint>;
}

impl DocExtManual for Doc {
    fn get_toc(&self) -> Vec<NavPoint> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(gepub_sys::gepub_doc_get_toc(self.to_glib_none().0))
        }
    }
}
