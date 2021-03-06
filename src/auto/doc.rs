// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gepub_sys;
use glib;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use TextChunk;

glib_wrapper! {
    pub struct Doc(Object<gepub_sys::GepubDoc, gepub_sys::GepubDocClass, DocClass>);

    match fn {
        get_type => || gepub_sys::gepub_doc_get_type(),
    }
}

impl Doc {
    pub fn new(path: &str) -> Result<Doc, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gepub_sys::gepub_doc_new(path.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_chapter(&self) -> i32 {
        unsafe {
            gepub_sys::gepub_doc_get_chapter(self.to_glib_none().0)
        }
    }

    pub fn get_content(&self) -> Option<glib::Bytes> {
        unsafe {
            from_glib_none(gepub_sys::gepub_doc_get_content(self.to_glib_none().0))
        }
    }

    pub fn get_cover(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gepub_sys::gepub_doc_get_cover(self.to_glib_none().0))
        }
    }

    pub fn get_current(&self) -> Option<glib::Bytes> {
        unsafe {
            from_glib_full(gepub_sys::gepub_doc_get_current(self.to_glib_none().0))
        }
    }

    pub fn get_current_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gepub_sys::gepub_doc_get_current_id(self.to_glib_none().0))
        }
    }

    pub fn get_current_mime(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gepub_sys::gepub_doc_get_current_mime(self.to_glib_none().0))
        }
    }

    pub fn get_current_path(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gepub_sys::gepub_doc_get_current_path(self.to_glib_none().0))
        }
    }

    pub fn get_current_with_epub_uris(&self) -> Option<glib::Bytes> {
        unsafe {
            from_glib_full(gepub_sys::gepub_doc_get_current_with_epub_uris(self.to_glib_none().0))
        }
    }

    pub fn get_metadata(&self, mdata: &str) -> Option<GString> {
        unsafe {
            from_glib_full(gepub_sys::gepub_doc_get_metadata(self.to_glib_none().0, mdata.to_glib_none().0))
        }
    }

    pub fn get_n_chapters(&self) -> i32 {
        unsafe {
            gepub_sys::gepub_doc_get_n_chapters(self.to_glib_none().0)
        }
    }

    pub fn get_resource(&self, path: &str) -> Option<glib::Bytes> {
        unsafe {
            from_glib_full(gepub_sys::gepub_doc_get_resource(self.to_glib_none().0, path.to_glib_none().0))
        }
    }

    pub fn get_resource_by_id(&self, id: &str) -> Option<glib::Bytes> {
        unsafe {
            from_glib_full(gepub_sys::gepub_doc_get_resource_by_id(self.to_glib_none().0, id.to_glib_none().0))
        }
    }

    pub fn get_resource_mime(&self, path: &str) -> Option<GString> {
        unsafe {
            from_glib_full(gepub_sys::gepub_doc_get_resource_mime(self.to_glib_none().0, path.to_glib_none().0))
        }
    }

    pub fn get_resource_mime_by_id(&self, id: &str) -> Option<GString> {
        unsafe {
            from_glib_full(gepub_sys::gepub_doc_get_resource_mime_by_id(self.to_glib_none().0, id.to_glib_none().0))
        }
    }

    pub fn get_resource_path(&self, id: &str) -> Option<GString> {
        unsafe {
            from_glib_full(gepub_sys::gepub_doc_get_resource_path(self.to_glib_none().0, id.to_glib_none().0))
        }
    }

    //pub fn get_resources(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 1, id: 3 } {
    //    unsafe { TODO: call gepub_sys:gepub_doc_get_resources() }
    //}

    pub fn get_text(&self) -> Vec<TextChunk> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gepub_sys::gepub_doc_get_text(self.to_glib_none().0))
        }
    }

    pub fn get_text_by_id(&self, id: &str) -> Vec<TextChunk> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gepub_sys::gepub_doc_get_text_by_id(self.to_glib_none().0, id.to_glib_none().0))
        }
    }

    //pub fn get_toc(&self) -> /*Ignored*/Vec<NavPoint> {
    //    unsafe { TODO: call gepub_sys:gepub_doc_get_toc() }
    //}

    pub fn go_next(&self) -> bool {
        unsafe {
            from_glib(gepub_sys::gepub_doc_go_next(self.to_glib_none().0))
        }
    }

    pub fn go_prev(&self) -> bool {
        unsafe {
            from_glib(gepub_sys::gepub_doc_go_prev(self.to_glib_none().0))
        }
    }

    pub fn resource_id_to_chapter(&self, id: &str) -> i32 {
        unsafe {
            gepub_sys::gepub_doc_resource_id_to_chapter(self.to_glib_none().0, id.to_glib_none().0)
        }
    }

    pub fn resource_uri_to_chapter(&self, uri: &str) -> i32 {
        unsafe {
            gepub_sys::gepub_doc_resource_uri_to_chapter(self.to_glib_none().0, uri.to_glib_none().0)
        }
    }

    pub fn set_chapter(&self, index: i32) {
        unsafe {
            gepub_sys::gepub_doc_set_chapter(self.to_glib_none().0, index);
        }
    }

    pub fn get_property_path(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"path\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `path` getter")
        }
    }

    pub fn connect_property_chapter_notify<F: Fn(&Doc) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_chapter_trampoline<F: Fn(&Doc) + 'static>(this: *mut gepub_sys::GepubDoc, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::chapter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_chapter_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Doc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Doc")
    }
}
