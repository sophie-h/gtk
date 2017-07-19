// This file was generated by gir (a4dcfea) from gir-files (0bcaef9)
// DO NOT EDIT

use Bin;
use Container;
use PackType;
use Widget;
use ffi;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ActionBar(Object<ffi::GtkActionBar>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_action_bar_get_type(),
    }
}

impl ActionBar {
    #[cfg(feature = "v3_12")]
    pub fn new() -> ActionBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_action_bar_new()).downcast_unchecked()
        }
    }
}

pub trait ActionBarExt {
    #[cfg(feature = "v3_12")]
    fn get_center_widget(&self) -> Option<Widget>;

    #[cfg(feature = "v3_12")]
    fn pack_end<P: IsA<Widget>>(&self, child: &P);

    #[cfg(feature = "v3_12")]
    fn pack_start<P: IsA<Widget>>(&self, child: &P);

    #[cfg(feature = "v3_12")]
    fn set_center_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, center_widget: Q);

    fn get_child_pack_type<T: IsA<Widget>>(&self, item: &T) -> PackType;

    fn set_child_pack_type<T: IsA<Widget>>(&self, item: &T, pack_type: PackType);

    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32);
}

impl<O: IsA<ActionBar> + IsA<Container>> ActionBarExt for O {
    #[cfg(feature = "v3_12")]
    fn get_center_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_action_bar_get_center_widget(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    fn pack_end<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_action_bar_pack_end(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    fn pack_start<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_action_bar_pack_start(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    fn set_center_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, center_widget: Q) {
        let center_widget = center_widget.into();
        let center_widget = center_widget.to_glib_none();
        unsafe {
            ffi::gtk_action_bar_set_center_widget(self.to_glib_none().0, center_widget.0);
        }
    }

    fn get_child_pack_type<T: IsA<Widget>>(&self, item: &T) -> PackType {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "pack-type".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    fn set_child_pack_type<T: IsA<Widget>>(&self, item: &T, pack_type: PackType) {
        let pack_type = pack_type.to_glib() as i32;
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "pack-type".to_glib_none().0, Value::from(&pack_type).to_glib_none().0);
        }
    }

    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "position".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "position".to_glib_none().0, Value::from(&position).to_glib_none().0);
        }
    }
}
