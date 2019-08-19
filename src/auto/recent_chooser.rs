// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;
use Error;
use RecentFilter;
use RecentInfo;
use RecentSortType;

glib_wrapper! {
    pub struct RecentChooser(Interface<gtk_sys::GtkRecentChooser>);

    match fn {
        get_type => || gtk_sys::gtk_recent_chooser_get_type(),
    }
}

pub const NONE_RECENT_CHOOSER: Option<&RecentChooser> = None;

pub trait RecentChooserExt: 'static {
    fn add_filter(&self, filter: &RecentFilter);

    fn get_current_item(&self) -> Option<RecentInfo>;

    fn get_current_uri(&self) -> Option<GString>;

    fn get_filter(&self) -> Option<RecentFilter>;

    fn get_items(&self) -> Vec<RecentInfo>;

    fn get_limit(&self) -> i32;

    fn get_local_only(&self) -> bool;

    fn get_select_multiple(&self) -> bool;

    fn get_show_icons(&self) -> bool;

    fn get_show_not_found(&self) -> bool;

    fn get_show_private(&self) -> bool;

    fn get_show_tips(&self) -> bool;

    fn get_sort_type(&self) -> RecentSortType;

    fn get_uris(&self) -> Vec<GString>;

    fn list_filters(&self) -> Vec<RecentFilter>;

    fn remove_filter(&self, filter: &RecentFilter);

    fn select_all(&self);

    fn select_uri(&self, uri: &str) -> Result<(), Error>;

    fn set_current_uri(&self, uri: &str) -> Result<(), Error>;

    fn set_filter(&self, filter: Option<&RecentFilter>);

    fn set_limit(&self, limit: i32);

    fn set_local_only(&self, local_only: bool);

    fn set_select_multiple(&self, select_multiple: bool);

    fn set_show_icons(&self, show_icons: bool);

    fn set_show_not_found(&self, show_not_found: bool);

    fn set_show_private(&self, show_private: bool);

    fn set_show_tips(&self, show_tips: bool);

    fn set_sort_func<P: Fn(&RecentInfo, &RecentInfo) -> i32 + 'static>(&self, sort_func: P);

    fn set_sort_type(&self, sort_type: RecentSortType);

    fn unselect_all(&self);

    fn unselect_uri(&self, uri: &str);

    fn connect_item_activated<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_limit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_local_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_select_multiple_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_show_icons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_not_found_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_show_private_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_show_tips_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_sort_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RecentChooser>> RecentChooserExt for O {
    fn add_filter(&self, filter: &RecentFilter) {
        unsafe {
            gtk_sys::gtk_recent_chooser_add_filter(
                self.as_ref().to_glib_none().0,
                filter.to_glib_none().0,
            );
        }
    }

    fn get_current_item(&self) -> Option<RecentInfo> {
        unsafe {
            from_glib_full(gtk_sys::gtk_recent_chooser_get_current_item(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_current_uri(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_sys::gtk_recent_chooser_get_current_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_filter(&self) -> Option<RecentFilter> {
        unsafe {
            from_glib_none(gtk_sys::gtk_recent_chooser_get_filter(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_items(&self) -> Vec<RecentInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gtk_sys::gtk_recent_chooser_get_items(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_limit(&self) -> i32 {
        unsafe { gtk_sys::gtk_recent_chooser_get_limit(self.as_ref().to_glib_none().0) }
    }

    fn get_local_only(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_recent_chooser_get_local_only(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_select_multiple(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_recent_chooser_get_select_multiple(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_show_icons(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_recent_chooser_get_show_icons(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_show_not_found(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_recent_chooser_get_show_not_found(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_show_private(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_recent_chooser_get_show_private(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_show_tips(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_recent_chooser_get_show_tips(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_sort_type(&self) -> RecentSortType {
        unsafe {
            from_glib(gtk_sys::gtk_recent_chooser_get_sort_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_uris(&self) -> Vec<GString> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(
                gtk_sys::gtk_recent_chooser_get_uris(
                    self.as_ref().to_glib_none().0,
                    length.as_mut_ptr(),
                ),
                length.assume_init() as usize,
            );
            ret
        }
    }

    fn list_filters(&self) -> Vec<RecentFilter> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gtk_sys::gtk_recent_chooser_list_filters(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove_filter(&self, filter: &RecentFilter) {
        unsafe {
            gtk_sys::gtk_recent_chooser_remove_filter(
                self.as_ref().to_glib_none().0,
                filter.to_glib_none().0,
            );
        }
    }

    fn select_all(&self) {
        unsafe {
            gtk_sys::gtk_recent_chooser_select_all(self.as_ref().to_glib_none().0);
        }
    }

    fn select_uri(&self, uri: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_recent_chooser_select_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_current_uri(&self, uri: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_recent_chooser_set_current_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_filter(&self, filter: Option<&RecentFilter>) {
        unsafe {
            gtk_sys::gtk_recent_chooser_set_filter(
                self.as_ref().to_glib_none().0,
                filter.to_glib_none().0,
            );
        }
    }

    fn set_limit(&self, limit: i32) {
        unsafe {
            gtk_sys::gtk_recent_chooser_set_limit(self.as_ref().to_glib_none().0, limit);
        }
    }

    fn set_local_only(&self, local_only: bool) {
        unsafe {
            gtk_sys::gtk_recent_chooser_set_local_only(
                self.as_ref().to_glib_none().0,
                local_only.to_glib(),
            );
        }
    }

    fn set_select_multiple(&self, select_multiple: bool) {
        unsafe {
            gtk_sys::gtk_recent_chooser_set_select_multiple(
                self.as_ref().to_glib_none().0,
                select_multiple.to_glib(),
            );
        }
    }

    fn set_show_icons(&self, show_icons: bool) {
        unsafe {
            gtk_sys::gtk_recent_chooser_set_show_icons(
                self.as_ref().to_glib_none().0,
                show_icons.to_glib(),
            );
        }
    }

    fn set_show_not_found(&self, show_not_found: bool) {
        unsafe {
            gtk_sys::gtk_recent_chooser_set_show_not_found(
                self.as_ref().to_glib_none().0,
                show_not_found.to_glib(),
            );
        }
    }

    fn set_show_private(&self, show_private: bool) {
        unsafe {
            gtk_sys::gtk_recent_chooser_set_show_private(
                self.as_ref().to_glib_none().0,
                show_private.to_glib(),
            );
        }
    }

    fn set_show_tips(&self, show_tips: bool) {
        unsafe {
            gtk_sys::gtk_recent_chooser_set_show_tips(
                self.as_ref().to_glib_none().0,
                show_tips.to_glib(),
            );
        }
    }

    fn set_sort_func<P: Fn(&RecentInfo, &RecentInfo) -> i32 + 'static>(&self, sort_func: P) {
        let sort_func_data: Box_<P> = Box_::new(sort_func);
        unsafe extern "C" fn sort_func_func<P: Fn(&RecentInfo, &RecentInfo) -> i32 + 'static>(
            a: *mut gtk_sys::GtkRecentInfo,
            b: *mut gtk_sys::GtkRecentInfo,
            user_data: glib_sys::gpointer,
        ) -> libc::c_int {
            let a = from_glib_borrow(a);
            let b = from_glib_borrow(b);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&a, &b);
            res
        }
        let sort_func = Some(sort_func_func::<P> as _);
        unsafe extern "C" fn data_destroy_func<P: Fn(&RecentInfo, &RecentInfo) -> i32 + 'static>(
            data: glib_sys::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(data_destroy_func::<P> as _);
        let super_callback0: Box_<P> = sort_func_data;
        unsafe {
            gtk_sys::gtk_recent_chooser_set_sort_func(
                self.as_ref().to_glib_none().0,
                sort_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    fn set_sort_type(&self, sort_type: RecentSortType) {
        unsafe {
            gtk_sys::gtk_recent_chooser_set_sort_type(
                self.as_ref().to_glib_none().0,
                sort_type.to_glib(),
            );
        }
    }

    fn unselect_all(&self) {
        unsafe {
            gtk_sys::gtk_recent_chooser_unselect_all(self.as_ref().to_glib_none().0);
        }
    }

    fn unselect_uri(&self, uri: &str) {
        unsafe {
            gtk_sys::gtk_recent_chooser_unselect_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            );
        }
    }

    fn connect_item_activated<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn item_activated_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRecentChooser,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RecentChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&RecentChooser::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"item-activated\0".as_ptr() as *const _,
                Some(transmute(item_activated_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn selection_changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRecentChooser,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RecentChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&RecentChooser::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"selection-changed\0".as_ptr() as *const _,
                Some(transmute(selection_changed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_filter_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRecentChooser,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RecentChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&RecentChooser::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::filter\0".as_ptr() as *const _,
                Some(transmute(notify_filter_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_limit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_limit_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRecentChooser,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RecentChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&RecentChooser::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::limit\0".as_ptr() as *const _,
                Some(transmute(notify_limit_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_local_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_local_only_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRecentChooser,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RecentChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&RecentChooser::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::local-only\0".as_ptr() as *const _,
                Some(transmute(notify_local_only_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_select_multiple_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_select_multiple_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRecentChooser,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RecentChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&RecentChooser::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::select-multiple\0".as_ptr() as *const _,
                Some(transmute(
                    notify_select_multiple_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_show_icons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_icons_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRecentChooser,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RecentChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&RecentChooser::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-icons\0".as_ptr() as *const _,
                Some(transmute(notify_show_icons_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_show_not_found_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_not_found_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRecentChooser,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RecentChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&RecentChooser::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-not-found\0".as_ptr() as *const _,
                Some(transmute(
                    notify_show_not_found_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_show_private_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_private_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRecentChooser,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RecentChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&RecentChooser::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-private\0".as_ptr() as *const _,
                Some(transmute(
                    notify_show_private_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_show_tips_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_tips_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRecentChooser,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RecentChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&RecentChooser::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-tips\0".as_ptr() as *const _,
                Some(transmute(notify_show_tips_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_sort_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sort_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRecentChooser,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RecentChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&RecentChooser::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::sort-type\0".as_ptr() as *const _,
                Some(transmute(notify_sort_type_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for RecentChooser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RecentChooser")
    }
}
