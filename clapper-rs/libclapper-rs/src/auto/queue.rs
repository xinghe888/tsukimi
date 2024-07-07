// This file was generated by gir (https://github.com/gtk-rs/gir)
// from 
// from ../gir-files-gstreamer
// from ../gir-files-gtk
// DO NOT EDIT

use crate::{MediaItem,QueueProgressionMode};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "ClapperQueue")]
    pub struct Queue(Object<ffi::ClapperQueue, ffi::ClapperQueueClass>) @extends gst::Object, @implements gio::ListModel;

    match fn {
        type_ => || ffi::clapper_queue_get_type(),
    }
}

impl Queue {
            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Queue`] objects.
            ///
            /// This method returns an instance of [`QueueBuilder`](crate::builders::QueueBuilder) which can be used to create [`Queue`] objects.
            pub fn builder() -> QueueBuilder {
                QueueBuilder::new()
            }
        

    #[doc(alias = "clapper_queue_add_item")]
    pub fn add_item(&self, item: &MediaItem) {
        unsafe {
            ffi::clapper_queue_add_item(self.to_glib_none().0, item.to_glib_none().0);
        }
    }

    #[doc(alias = "clapper_queue_clear")]
    pub fn clear(&self) {
        unsafe {
            ffi::clapper_queue_clear(self.to_glib_none().0);
        }
    }

    #[doc(alias = "clapper_queue_find_item")]
    pub fn find_item(&self, item: &MediaItem) -> Option<u32> {
        unsafe {
            let mut index = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::clapper_queue_find_item(self.to_glib_none().0, item.to_glib_none().0, index.as_mut_ptr()));
            if ret { Some(index.assume_init()) } else { None }
        }
    }

    #[doc(alias = "clapper_queue_get_current_index")]
    #[doc(alias = "get_current_index")]
    pub fn current_index(&self) -> u32 {
        unsafe {
            ffi::clapper_queue_get_current_index(self.to_glib_none().0)
        }
    }

    #[doc(alias = "clapper_queue_get_current_item")]
    #[doc(alias = "get_current_item")]
    pub fn current_item(&self) -> Option<MediaItem> {
        unsafe {
            from_glib_full(ffi::clapper_queue_get_current_item(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_queue_get_gapless")]
    #[doc(alias = "get_gapless")]
    pub fn is_gapless(&self) -> bool {
        unsafe {
            from_glib(ffi::clapper_queue_get_gapless(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_queue_get_instant")]
    #[doc(alias = "get_instant")]
    pub fn is_instant(&self) -> bool {
        unsafe {
            from_glib(ffi::clapper_queue_get_instant(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_queue_get_item")]
    #[doc(alias = "get_item")]
    pub fn item(&self, index: u32) -> Option<MediaItem> {
        unsafe {
            from_glib_full(ffi::clapper_queue_get_item(self.to_glib_none().0, index))
        }
    }

    #[doc(alias = "clapper_queue_get_n_items")]
    #[doc(alias = "get_n_items")]
    pub fn n_items(&self) -> u32 {
        unsafe {
            ffi::clapper_queue_get_n_items(self.to_glib_none().0)
        }
    }

    #[doc(alias = "clapper_queue_get_progression_mode")]
    #[doc(alias = "get_progression_mode")]
    pub fn progression_mode(&self) -> QueueProgressionMode {
        unsafe {
            from_glib(ffi::clapper_queue_get_progression_mode(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_queue_insert_item")]
    pub fn insert_item(&self, item: &MediaItem, index: i32) {
        unsafe {
            ffi::clapper_queue_insert_item(self.to_glib_none().0, item.to_glib_none().0, index);
        }
    }

    #[doc(alias = "clapper_queue_item_is_current")]
    pub fn item_is_current(&self, item: &MediaItem) -> bool {
        unsafe {
            from_glib(ffi::clapper_queue_item_is_current(self.to_glib_none().0, item.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_queue_remove_index")]
    pub fn remove_index(&self, index: u32) {
        unsafe {
            ffi::clapper_queue_remove_index(self.to_glib_none().0, index);
        }
    }

    #[doc(alias = "clapper_queue_remove_item")]
    pub fn remove_item(&self, item: &MediaItem) {
        unsafe {
            ffi::clapper_queue_remove_item(self.to_glib_none().0, item.to_glib_none().0);
        }
    }

    #[doc(alias = "clapper_queue_reposition_item")]
    pub fn reposition_item(&self, item: &MediaItem, index: i32) {
        unsafe {
            ffi::clapper_queue_reposition_item(self.to_glib_none().0, item.to_glib_none().0, index);
        }
    }

    #[doc(alias = "clapper_queue_select_index")]
    pub fn select_index(&self, index: u32) -> bool {
        unsafe {
            from_glib(ffi::clapper_queue_select_index(self.to_glib_none().0, index))
        }
    }

    #[doc(alias = "clapper_queue_select_item")]
    pub fn select_item(&self, item: Option<&MediaItem>) -> bool {
        unsafe {
            from_glib(ffi::clapper_queue_select_item(self.to_glib_none().0, item.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_queue_select_next_item")]
    pub fn select_next_item(&self) -> bool {
        unsafe {
            from_glib(ffi::clapper_queue_select_next_item(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_queue_select_previous_item")]
    pub fn select_previous_item(&self) -> bool {
        unsafe {
            from_glib(ffi::clapper_queue_select_previous_item(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_queue_set_gapless")]
    pub fn set_gapless(&self, gapless: bool) {
        unsafe {
            ffi::clapper_queue_set_gapless(self.to_glib_none().0, gapless.into_glib());
        }
    }

    #[doc(alias = "clapper_queue_set_instant")]
    pub fn set_instant(&self, instant: bool) {
        unsafe {
            ffi::clapper_queue_set_instant(self.to_glib_none().0, instant.into_glib());
        }
    }

    #[doc(alias = "clapper_queue_set_progression_mode")]
    pub fn set_progression_mode(&self, mode: QueueProgressionMode) {
        unsafe {
            ffi::clapper_queue_set_progression_mode(self.to_glib_none().0, mode.into_glib());
        }
    }

    #[doc(alias = "clapper_queue_steal_index")]
    pub fn steal_index(&self, index: u32) -> Option<MediaItem> {
        unsafe {
            from_glib_full(ffi::clapper_queue_steal_index(self.to_glib_none().0, index))
        }
    }

    #[doc(alias = "current-index")]
    pub fn set_current_index(&self, current_index: u32) {
        ObjectExt::set_property(self,"current-index", current_index)
    }

    #[doc(alias = "current-index")]
    pub fn connect_current_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_current_index_trampoline<F: Fn(&Queue) + 'static>(this: *mut ffi::ClapperQueue, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::current-index\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_current_index_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "current-item")]
    pub fn connect_current_item_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_current_item_trampoline<F: Fn(&Queue) + 'static>(this: *mut ffi::ClapperQueue, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::current-item\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_current_item_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "gapless")]
    pub fn connect_gapless_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gapless_trampoline<F: Fn(&Queue) + 'static>(this: *mut ffi::ClapperQueue, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::gapless\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_gapless_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "instant")]
    pub fn connect_instant_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_instant_trampoline<F: Fn(&Queue) + 'static>(this: *mut ffi::ClapperQueue, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::instant\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_instant_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "n-items")]
    pub fn connect_n_items_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_items_trampoline<F: Fn(&Queue) + 'static>(this: *mut ffi::ClapperQueue, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::n-items\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_n_items_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "progression-mode")]
    pub fn connect_progression_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_progression_mode_trampoline<F: Fn(&Queue) + 'static>(this: *mut ffi::ClapperQueue, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::progression-mode\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_progression_mode_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Queue`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct QueueBuilder {
            builder: glib::object::ObjectBuilder<'static, Queue>,
        }

        impl QueueBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn current_index(self, current_index: u32) -> Self {
                            Self { builder: self.builder.property("current-index", current_index), }
                        }

                            pub fn gapless(self, gapless: bool) -> Self {
                            Self { builder: self.builder.property("gapless", gapless), }
                        }

                            pub fn instant(self, instant: bool) -> Self {
                            Self { builder: self.builder.property("instant", instant), }
                        }

                            pub fn progression_mode(self, progression_mode: QueueProgressionMode) -> Self {
                            Self { builder: self.builder.property("progression-mode", progression_mode), }
                        }

                            pub fn name(self, name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("name", name.into()), }
                        }

                            pub fn parent(self, parent: &impl IsA<gst::Object>) -> Self {
                            Self { builder: self.builder.property("parent", parent.clone().upcast()), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`Queue`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Queue {
    self.builder.build() }
}
