// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::approx_constant, clippy::type_complexity, clippy::unreadable_literal)]

extern crate libc;
extern crate gtk_sys as gtk;
extern crate glib_sys as glib;
extern crate gobject_sys as gobject;
extern crate webkit2gtk_sys as webkit2gtk;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type GepubTextChunkType = c_int;
pub const GEPUBTextHeader: GepubTextChunkType = 0;
pub const GEPUBTextBold: GepubTextChunkType = 1;
pub const GEPUBTextItalic: GepubTextChunkType = 2;
pub const GEPUBTextNormal: GepubTextChunkType = 3;

// Constants
pub const GEPUB_META_AUTHOR: *const c_char = b"creator\0" as *const u8 as *const c_char;
pub const GEPUB_META_DESC: *const c_char = b"description\0" as *const u8 as *const c_char;
pub const GEPUB_META_ID: *const c_char = b"identifier\0" as *const u8 as *const c_char;
pub const GEPUB_META_LANG: *const c_char = b"language\0" as *const u8 as *const c_char;
pub const GEPUB_META_TITLE: *const c_char = b"title\0" as *const u8 as *const c_char;

// Records
#[repr(C)]
pub struct _GepubArchiveClass(c_void);

pub type GepubArchiveClass = *mut _GepubArchiveClass;

#[repr(C)]
pub struct _GepubDocClass(c_void);

pub type GepubDocClass = *mut _GepubDocClass;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GepubNavPoint {
    pub label: *mut c_char,
    pub content: *mut c_char,
    pub playorder: u64,
}

impl ::std::fmt::Debug for GepubNavPoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GepubNavPoint @ {:?}", self as *const _))
         .field("label", &self.label)
         .field("content", &self.content)
         .field("playorder", &self.playorder)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GepubResource {
    pub mime: *mut c_char,
    pub uri: *mut c_char,
}

impl ::std::fmt::Debug for GepubResource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GepubResource @ {:?}", self as *const _))
         .field("mime", &self.mime)
         .field("uri", &self.uri)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GepubTextChunkClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for GepubTextChunkClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GepubTextChunkClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[repr(C)]
pub struct _GepubWidgetClass(c_void);

pub type GepubWidgetClass = *mut _GepubWidgetClass;

// Classes
#[repr(C)]
pub struct GepubArchive(c_void);

impl ::std::fmt::Debug for GepubArchive {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GepubArchive @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
pub struct GepubDoc(c_void);

impl ::std::fmt::Debug for GepubDoc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GepubDoc @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GepubTextChunk {
    pub parent: gobject::GObject,
    pub type_: GepubTextChunkType,
    pub text: *mut c_char,
}

impl ::std::fmt::Debug for GepubTextChunk {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GepubTextChunk @ {:?}", self as *const _))
         .field("parent", &self.parent)
         .field("type_", &self.type_)
         .field("text", &self.text)
         .finish()
    }
}

#[repr(C)]
pub struct GepubWidget(c_void);

impl ::std::fmt::Debug for GepubWidget {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GepubWidget @ {:?}", self as *const _))
         .finish()
    }
}

#[link(name = "gepub-0.6")]
extern "C" {

    //=========================================================================
    // GepubArchive
    //=========================================================================
    pub fn gepub_archive_get_type() -> GType;
    pub fn gepub_archive_new(path: *const c_char) -> *mut GepubArchive;
    pub fn gepub_archive_get_root_file(archive: *mut GepubArchive) -> *mut c_char;
    pub fn gepub_archive_list_files(archive: *mut GepubArchive) -> *mut glib::GList;
    pub fn gepub_archive_read_entry(archive: *mut GepubArchive, path: *const c_char) -> *mut glib::GBytes;

    //=========================================================================
    // GepubDoc
    //=========================================================================
    pub fn gepub_doc_get_type() -> GType;
    pub fn gepub_doc_new(path: *const c_char, error: *mut *mut glib::GError) -> *mut GepubDoc;
    pub fn gepub_doc_get_chapter(doc: *mut GepubDoc) -> c_int;
    pub fn gepub_doc_get_content(doc: *mut GepubDoc) -> *mut glib::GBytes;
    pub fn gepub_doc_get_cover(doc: *mut GepubDoc) -> *mut c_char;
    pub fn gepub_doc_get_current(doc: *mut GepubDoc) -> *mut glib::GBytes;
    pub fn gepub_doc_get_current_id(doc: *mut GepubDoc) -> *const c_char;
    pub fn gepub_doc_get_current_mime(doc: *mut GepubDoc) -> *mut c_char;
    pub fn gepub_doc_get_current_path(doc: *mut GepubDoc) -> *mut c_char;
    pub fn gepub_doc_get_current_with_epub_uris(doc: *mut GepubDoc) -> *mut glib::GBytes;
    pub fn gepub_doc_get_metadata(doc: *mut GepubDoc, mdata: *const c_char) -> *mut c_char;
    pub fn gepub_doc_get_n_chapters(doc: *mut GepubDoc) -> c_int;
    pub fn gepub_doc_get_resource(doc: *mut GepubDoc, path: *const c_char) -> *mut glib::GBytes;
    pub fn gepub_doc_get_resource_by_id(doc: *mut GepubDoc, id: *const c_char) -> *mut glib::GBytes;
    pub fn gepub_doc_get_resource_mime(doc: *mut GepubDoc, path: *const c_char) -> *mut c_char;
    pub fn gepub_doc_get_resource_mime_by_id(doc: *mut GepubDoc, id: *const c_char) -> *mut c_char;
    pub fn gepub_doc_get_resource_path(doc: *mut GepubDoc, id: *const c_char) -> *mut c_char;
    pub fn gepub_doc_get_resources(doc: *mut GepubDoc) -> *mut glib::GHashTable;
    pub fn gepub_doc_get_text(doc: *mut GepubDoc) -> *mut glib::GList;
    pub fn gepub_doc_get_text_by_id(doc: *mut GepubDoc, id: *const c_char) -> *mut glib::GList;
    pub fn gepub_doc_get_toc(doc: *mut GepubDoc) -> *mut glib::GList;
    pub fn gepub_doc_go_next(doc: *mut GepubDoc) -> gboolean;
    pub fn gepub_doc_go_prev(doc: *mut GepubDoc) -> gboolean;
    pub fn gepub_doc_resource_id_to_chapter(doc: *mut GepubDoc, id: *const c_char) -> c_int;
    pub fn gepub_doc_resource_uri_to_chapter(doc: *mut GepubDoc, uri: *const c_char) -> c_int;
    pub fn gepub_doc_set_chapter(doc: *mut GepubDoc, index: c_int);

    //=========================================================================
    // GepubTextChunk
    //=========================================================================
    pub fn gepub_text_chunk_get_type() -> GType;
    pub fn gepub_text_chunk_new(type_: GepubTextChunkType, text: *const c_char) -> *mut GepubTextChunk;
    pub fn gepub_text_chunk_text(chunk: *mut GepubTextChunk) -> *const c_char;
    pub fn gepub_text_chunk_type(chunk: *mut GepubTextChunk) -> GepubTextChunkType;
    pub fn gepub_text_chunk_type_str(chunk: *mut GepubTextChunk) -> *const c_char;

    //=========================================================================
    // GepubWidget
    //=========================================================================
    pub fn gepub_widget_get_type() -> GType;
    pub fn gepub_widget_new() -> *mut gtk::GtkWidget;
    pub fn gepub_widget_chapter_next(widget: *mut GepubWidget) -> gboolean;
    pub fn gepub_widget_chapter_prev(widget: *mut GepubWidget) -> gboolean;
    pub fn gepub_widget_get_chapter(widget: *mut GepubWidget) -> c_int;
    pub fn gepub_widget_get_chapter_length(widget: *mut GepubWidget) -> c_int;
    pub fn gepub_widget_get_doc(widget: *mut GepubWidget) -> *mut GepubDoc;
    pub fn gepub_widget_get_fontfamily(widget: *mut GepubWidget) -> *mut c_char;
    pub fn gepub_widget_get_fontsize(widget: *mut GepubWidget) -> c_int;
    pub fn gepub_widget_get_lineheight(widget: *mut GepubWidget) -> c_float;
    pub fn gepub_widget_get_margin(widget: *mut GepubWidget) -> c_int;
    pub fn gepub_widget_get_n_chapters(widget: *mut GepubWidget) -> c_int;
    pub fn gepub_widget_get_paginate(widget: *mut GepubWidget) -> gboolean;
    pub fn gepub_widget_get_pos(widget: *mut GepubWidget) -> c_float;
    pub fn gepub_widget_page_next(widget: *mut GepubWidget) -> gboolean;
    pub fn gepub_widget_page_prev(widget: *mut GepubWidget) -> gboolean;
    pub fn gepub_widget_set_chapter(widget: *mut GepubWidget, index: c_int);
    pub fn gepub_widget_set_doc(widget: *mut GepubWidget, doc: *mut GepubDoc);
    pub fn gepub_widget_set_fontfamily(widget: *mut GepubWidget, family: *mut c_char);
    pub fn gepub_widget_set_fontsize(widget: *mut GepubWidget, size: c_int);
    pub fn gepub_widget_set_lineheight(widget: *mut GepubWidget, size: c_float);
    pub fn gepub_widget_set_margin(widget: *mut GepubWidget, margin: c_int);
    pub fn gepub_widget_set_paginate(widget: *mut GepubWidget, p: gboolean);
    pub fn gepub_widget_set_pos(widget: *mut GepubWidget, index: c_float);

    //=========================================================================
    // Other functions
    //=========================================================================
    //pub fn gepub_utils_get_element_by_attr(node: /*Ignored*/*mut libxml2::xmlNode, attr: *const c_char, value: *const c_char) -> /*Ignored*/*mut libxml2::xmlNode;
    //pub fn gepub_utils_get_element_by_tag(node: /*Ignored*/*mut libxml2::xmlNode, name: *const c_char) -> /*Ignored*/*mut libxml2::xmlNode;
    //pub fn gepub_utils_get_prop(node: /*Ignored*/*mut libxml2::xmlNode, prop: *const c_char) -> *mut c_char;
    //pub fn gepub_utils_get_text_elements(node: /*Ignored*/*mut libxml2::xmlNode) -> *mut glib::GList;
    pub fn gepub_utils_replace_resources(content: *mut glib::GBytes, path: *const c_char) -> *mut glib::GBytes;

}
