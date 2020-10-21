use glib::GString;
use glib::translate::{
    FromGlibPtrFull,
    FromGlibPtrNone,
    GlibPtrDefault,
};

use gepub_sys::GepubNavPoint;

#[derive(Debug)]
pub struct NavPoint {
    pub label: GString,
    pub content: GString,
    pub playorder: usize,
}

impl GlibPtrDefault for NavPoint {
    type GlibType = *mut GepubNavPoint;
}

impl FromGlibPtrNone<*mut GepubNavPoint> for NavPoint {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut GepubNavPoint) -> Self {
        NavPoint {
            label: GString::from_glib_none((*ptr).label),
            content: GString::from_glib_none((*ptr).content),
            playorder: (*ptr).playorder as usize,
        }
    }
}

impl FromGlibPtrFull<*mut GepubNavPoint> for NavPoint {
    #[inline]
    unsafe fn from_glib_full(_ptr: *mut GepubNavPoint) -> Self {
        unimplemented!();
        /*NavPoint {
        }*/
    }
}
