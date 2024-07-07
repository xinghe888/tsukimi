// This file was generated by gir (https://github.com/gtk-rs/gir)
// from 
// from ../gir-files-gstreamer
// from ../gir-files-gtk
// DO NOT EDIT

use crate::{Feature};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "ClapperMpris")]
    pub struct Mpris(Object<ffi::ClapperMpris, ffi::ClapperMprisClass>) @extends Feature, gst::Object;

    match fn {
        type_ => || ffi::clapper_mpris_get_type(),
    }
}

impl Mpris {
    #[doc(alias = "clapper_mpris_new")]
    pub fn new(own_name: &str, identity: &str, desktop_entry: Option<&str>) -> Mpris {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::clapper_mpris_new(own_name.to_glib_none().0, identity.to_glib_none().0, desktop_entry.to_glib_none().0))
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Mpris`] objects.
            ///
            /// This method returns an instance of [`MprisBuilder`](crate::builders::MprisBuilder) which can be used to create [`Mpris`] objects.
            pub fn builder() -> MprisBuilder {
                MprisBuilder::new()
            }
        

    #[doc(alias = "clapper_mpris_get_fallback_art_url")]
    #[doc(alias = "get_fallback_art_url")]
    pub fn fallback_art_url(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::clapper_mpris_get_fallback_art_url(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_mpris_get_queue_controllable")]
    #[doc(alias = "get_queue_controllable")]
    pub fn is_queue_controllable(&self) -> bool {
        unsafe {
            from_glib(ffi::clapper_mpris_get_queue_controllable(self.to_glib_none().0))
        }
    }

    #[doc(alias = "clapper_mpris_set_fallback_art_url")]
    pub fn set_fallback_art_url(&self, art_url: Option<&str>) {
        unsafe {
            ffi::clapper_mpris_set_fallback_art_url(self.to_glib_none().0, art_url.to_glib_none().0);
        }
    }

    #[doc(alias = "clapper_mpris_set_queue_controllable")]
    pub fn set_queue_controllable(&self, controllable: bool) {
        unsafe {
            ffi::clapper_mpris_set_queue_controllable(self.to_glib_none().0, controllable.into_glib());
        }
    }

    #[doc(alias = "desktop-entry")]
    pub fn desktop_entry(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "desktop-entry")
    }

    pub fn identity(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "identity")
    }

    #[doc(alias = "own-name")]
    pub fn own_name(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "own-name")
    }

    #[doc(alias = "fallback-art-url")]
    pub fn connect_fallback_art_url_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_fallback_art_url_trampoline<F: Fn(&Mpris) + 'static>(this: *mut ffi::ClapperMpris, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::fallback-art-url\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_fallback_art_url_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "queue-controllable")]
    pub fn connect_queue_controllable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_queue_controllable_trampoline<F: Fn(&Mpris) + 'static>(this: *mut ffi::ClapperMpris, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::queue-controllable\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_queue_controllable_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl Default for Mpris {
                     fn default() -> Self {
                         glib::object::Object::new::<Self>()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Mpris`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct MprisBuilder {
            builder: glib::object::ObjectBuilder<'static, Mpris>,
        }

        impl MprisBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn desktop_entry(self, desktop_entry: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("desktop-entry", desktop_entry.into()), }
                        }

                            pub fn fallback_art_url(self, fallback_art_url: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("fallback-art-url", fallback_art_url.into()), }
                        }

                            pub fn identity(self, identity: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("identity", identity.into()), }
                        }

                            pub fn own_name(self, own_name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("own-name", own_name.into()), }
                        }

                            pub fn queue_controllable(self, queue_controllable: bool) -> Self {
                            Self { builder: self.builder.property("queue-controllable", queue_controllable), }
                        }

                            pub fn name(self, name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("name", name.into()), }
                        }

                            pub fn parent(self, parent: &impl IsA<gst::Object>) -> Self {
                            Self { builder: self.builder.property("parent", parent.clone().upcast()), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`Mpris`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Mpris {
    self.builder.build() }
}
