// This file was generated by gir (127a851) from gir-files (0bcaef9)
// DO NOT EDIT

use Actionable;
use Bin;
use Container;
use MenuItem;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct CheckMenuItem(Object<ffi::GtkCheckMenuItem>): MenuItem, Bin, Container, Widget, Actionable;

    match fn {
        get_type => || ffi::gtk_check_menu_item_get_type(),
    }
}

impl CheckMenuItem {
    pub fn new() -> CheckMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_check_menu_item_new()).downcast_unchecked()
        }
    }

    pub fn new_with_label(label: &str) -> CheckMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_check_menu_item_new_with_label(label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(label: &str) -> CheckMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_check_menu_item_new_with_mnemonic(label.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait CheckMenuItemExt {
    fn get_active(&self) -> bool;

    fn get_draw_as_radio(&self) -> bool;

    fn get_inconsistent(&self) -> bool;

    fn set_active(&self, is_active: bool);

    fn set_draw_as_radio(&self, draw_as_radio: bool);

    fn set_inconsistent(&self, setting: bool);

    fn toggled(&self);

    fn connect_toggled<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<CheckMenuItem> + IsA<glib::object::Object>> CheckMenuItemExt for O {
    fn get_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_check_menu_item_get_active(self.to_glib_none().0))
        }
    }

    fn get_draw_as_radio(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_check_menu_item_get_draw_as_radio(self.to_glib_none().0))
        }
    }

    fn get_inconsistent(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_check_menu_item_get_inconsistent(self.to_glib_none().0))
        }
    }

    fn set_active(&self, is_active: bool) {
        unsafe {
            ffi::gtk_check_menu_item_set_active(self.to_glib_none().0, is_active.to_glib());
        }
    }

    fn set_draw_as_radio(&self, draw_as_radio: bool) {
        unsafe {
            ffi::gtk_check_menu_item_set_draw_as_radio(self.to_glib_none().0, draw_as_radio.to_glib());
        }
    }

    fn set_inconsistent(&self, setting: bool) {
        unsafe {
            ffi::gtk_check_menu_item_set_inconsistent(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn toggled(&self) {
        unsafe {
            ffi::gtk_check_menu_item_toggled(self.to_glib_none().0);
        }
    }

    fn connect_toggled<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "toggled",
                transmute(toggled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn toggled_trampoline<P>(this: *mut ffi::GtkCheckMenuItem, f: glib_ffi::gpointer)
where P: IsA<CheckMenuItem> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&CheckMenuItem::from_glib_none(this).downcast_unchecked())
}
