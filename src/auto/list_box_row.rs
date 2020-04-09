// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use Actionable;
use Align;
use Bin;
use Buildable;
use Container;
use ResizeMode;
use Widget;

glib_wrapper! {
    pub struct ListBoxRow(Object<gtk_sys::GtkListBoxRow, gtk_sys::GtkListBoxRowClass, ListBoxRowClass>) @extends Bin, Container, Widget, @implements Buildable, Actionable;

    match fn {
        get_type => || gtk_sys::gtk_list_box_row_get_type(),
    }
}

impl ListBoxRow {
    pub fn new() -> ListBoxRow {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_list_box_row_new()).unsafe_cast() }
    }
}

impl Default for ListBoxRow {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct ListBoxRowBuilder {
    activatable: Option<bool>,
    selectable: Option<bool>,
    border_width: Option<u32>,
    child: Option<Widget>,
    resize_mode: Option<ResizeMode>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    action_name: Option<String>,
    action_target: Option<glib::Variant>,
}

impl ListBoxRowBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> ListBoxRow {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref activatable) = self.activatable {
            properties.push(("activatable", activatable));
        }
        if let Some(ref selectable) = self.selectable {
            properties.push(("selectable", selectable));
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref resize_mode) = self.resize_mode {
            properties.push(("resize-mode", resize_mode));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        {
            if let Some(ref focus_on_click) = self.focus_on_click {
                properties.push(("focus-on-click", focus_on_click));
            }
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref action_name) = self.action_name {
            properties.push(("action-name", action_name));
        }
        if let Some(ref action_target) = self.action_target {
            properties.push(("action-target", action_target));
        }
        glib::Object::new(ListBoxRow::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn activatable(mut self, activatable: bool) -> Self {
        self.activatable = Some(activatable);
        self
    }

    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = Some(selectable);
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child<P: IsA<Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn resize_mode(mut self, resize_mode: ResizeMode) -> Self {
        self.resize_mode = Some(resize_mode);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent<P: IsA<Container>>(mut self, parent: &P) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn action_name(mut self, action_name: &str) -> Self {
        self.action_name = Some(action_name.to_string());
        self
    }

    pub fn action_target(mut self, action_target: &glib::Variant) -> Self {
        self.action_target = Some(action_target.clone());
        self
    }
}

pub const NONE_LIST_BOX_ROW: Option<&ListBoxRow> = None;

pub trait ListBoxRowExt: 'static {
    fn changed(&self);

    fn get_activatable(&self) -> bool;

    fn get_header(&self) -> Option<Widget>;

    fn get_index(&self) -> i32;

    fn get_selectable(&self) -> bool;

    fn is_selected(&self) -> bool;

    fn set_activatable(&self, activatable: bool);

    fn set_header<P: IsA<Widget>>(&self, header: Option<&P>);

    fn set_selectable(&self, selectable: bool);

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate(&self);

    fn connect_property_activatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_selectable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ListBoxRow>> ListBoxRowExt for O {
    fn changed(&self) {
        unsafe {
            gtk_sys::gtk_list_box_row_changed(self.as_ref().to_glib_none().0);
        }
    }

    fn get_activatable(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_list_box_row_get_activatable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_header(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_list_box_row_get_header(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_index(&self) -> i32 {
        unsafe { gtk_sys::gtk_list_box_row_get_index(self.as_ref().to_glib_none().0) }
    }

    fn get_selectable(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_list_box_row_get_selectable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_selected(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_list_box_row_is_selected(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_activatable(&self, activatable: bool) {
        unsafe {
            gtk_sys::gtk_list_box_row_set_activatable(
                self.as_ref().to_glib_none().0,
                activatable.to_glib(),
            );
        }
    }

    fn set_header<P: IsA<Widget>>(&self, header: Option<&P>) {
        unsafe {
            gtk_sys::gtk_list_box_row_set_header(
                self.as_ref().to_glib_none().0,
                header.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_selectable(&self, selectable: bool) {
        unsafe {
            gtk_sys::gtk_list_box_row_set_selectable(
                self.as_ref().to_glib_none().0,
                selectable.to_glib(),
            );
        }
    }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkListBoxRow,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ListBoxRow>,
        {
            let f: &F = &*(f as *const F);
            f(&ListBoxRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(*(&activate_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_activate(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject)
                .emit("activate", &[])
                .unwrap()
        };
    }

    fn connect_property_activatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_activatable_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkListBoxRow,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ListBoxRow>,
        {
            let f: &F = &*(f as *const F);
            f(&ListBoxRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::activatable\0".as_ptr() as *const _,
                Some(*(&notify_activatable_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_selectable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selectable_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkListBoxRow,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ListBoxRow>,
        {
            let f: &F = &*(f as *const F);
            f(&ListBoxRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::selectable\0".as_ptr() as *const _,
                Some(*(&notify_selectable_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ListBoxRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ListBoxRow")
    }
}
