// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gtk_sys;
use pango;
use std::boxed::Box as Box_;
use std::fmt;
use Actionable;
use Align;
use Bin;
use Buildable;
use Button;
use Container;
use FontChooser;
#[cfg(any(feature = "v3_24", feature = "dox"))]
use FontChooserLevel;
use PositionType;
use ReliefStyle;
use ResizeMode;
use Widget;

glib_wrapper! {
    pub struct FontButton(Object<gtk_sys::GtkFontButton, gtk_sys::GtkFontButtonClass, FontButtonClass>) @extends Button, Bin, Container, Widget, @implements Buildable, Actionable, FontChooser;

    match fn {
        get_type => || gtk_sys::gtk_font_button_get_type(),
    }
}

impl FontButton {
    pub fn new() -> FontButton {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_font_button_new()).unsafe_cast() }
    }

    pub fn new_with_font(fontname: &str) -> FontButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_font_button_new_with_font(
                fontname.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

impl Default for FontButton {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct FontButtonBuilder {
    font_name: Option<String>,
    show_size: Option<bool>,
    show_style: Option<bool>,
    title: Option<String>,
    use_font: Option<bool>,
    use_size: Option<bool>,
    always_show_image: Option<bool>,
    image: Option<Widget>,
    image_position: Option<PositionType>,
    label: Option<String>,
    relief: Option<ReliefStyle>,
    use_underline: Option<bool>,
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
    font: Option<String>,
    font_desc: Option<pango::FontDescription>,
    #[cfg(any(feature = "v3_24", feature = "dox"))]
    language: Option<String>,
    #[cfg(any(feature = "v3_24", feature = "dox"))]
    level: Option<FontChooserLevel>,
    preview_text: Option<String>,
    show_preview_entry: Option<bool>,
}

impl FontButtonBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> FontButton {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref font_name) = self.font_name {
            properties.push(("font-name", font_name));
        }
        if let Some(ref show_size) = self.show_size {
            properties.push(("show-size", show_size));
        }
        if let Some(ref show_style) = self.show_style {
            properties.push(("show-style", show_style));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        if let Some(ref use_font) = self.use_font {
            properties.push(("use-font", use_font));
        }
        if let Some(ref use_size) = self.use_size {
            properties.push(("use-size", use_size));
        }
        if let Some(ref always_show_image) = self.always_show_image {
            properties.push(("always-show-image", always_show_image));
        }
        if let Some(ref image) = self.image {
            properties.push(("image", image));
        }
        if let Some(ref image_position) = self.image_position {
            properties.push(("image-position", image_position));
        }
        if let Some(ref label) = self.label {
            properties.push(("label", label));
        }
        if let Some(ref relief) = self.relief {
            properties.push(("relief", relief));
        }
        if let Some(ref use_underline) = self.use_underline {
            properties.push(("use-underline", use_underline));
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
        if let Some(ref font) = self.font {
            properties.push(("font", font));
        }
        if let Some(ref font_desc) = self.font_desc {
            properties.push(("font-desc", font_desc));
        }
        #[cfg(any(feature = "v3_24", feature = "dox"))]
        {
            if let Some(ref language) = self.language {
                properties.push(("language", language));
            }
        }
        #[cfg(any(feature = "v3_24", feature = "dox"))]
        {
            if let Some(ref level) = self.level {
                properties.push(("level", level));
            }
        }
        if let Some(ref preview_text) = self.preview_text {
            properties.push(("preview-text", preview_text));
        }
        if let Some(ref show_preview_entry) = self.show_preview_entry {
            properties.push(("show-preview-entry", show_preview_entry));
        }
        glib::Object::new(FontButton::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn font_name(mut self, font_name: &str) -> Self {
        self.font_name = Some(font_name.to_string());
        self
    }

    pub fn show_size(mut self, show_size: bool) -> Self {
        self.show_size = Some(show_size);
        self
    }

    pub fn show_style(mut self, show_style: bool) -> Self {
        self.show_style = Some(show_style);
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn use_font(mut self, use_font: bool) -> Self {
        self.use_font = Some(use_font);
        self
    }

    pub fn use_size(mut self, use_size: bool) -> Self {
        self.use_size = Some(use_size);
        self
    }

    pub fn always_show_image(mut self, always_show_image: bool) -> Self {
        self.always_show_image = Some(always_show_image);
        self
    }

    pub fn image<P: IsA<Widget>>(mut self, image: &P) -> Self {
        self.image = Some(image.clone().upcast());
        self
    }

    pub fn image_position(mut self, image_position: PositionType) -> Self {
        self.image_position = Some(image_position);
        self
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn relief(mut self, relief: ReliefStyle) -> Self {
        self.relief = Some(relief);
        self
    }

    pub fn use_underline(mut self, use_underline: bool) -> Self {
        self.use_underline = Some(use_underline);
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

    pub fn font(mut self, font: &str) -> Self {
        self.font = Some(font.to_string());
        self
    }

    pub fn font_desc(mut self, font_desc: &pango::FontDescription) -> Self {
        self.font_desc = Some(font_desc.clone());
        self
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    pub fn language(mut self, language: &str) -> Self {
        self.language = Some(language.to_string());
        self
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    pub fn level(mut self, level: FontChooserLevel) -> Self {
        self.level = Some(level);
        self
    }

    pub fn preview_text(mut self, preview_text: &str) -> Self {
        self.preview_text = Some(preview_text.to_string());
        self
    }

    pub fn show_preview_entry(mut self, show_preview_entry: bool) -> Self {
        self.show_preview_entry = Some(show_preview_entry);
        self
    }
}

pub const NONE_FONT_BUTTON: Option<&FontButton> = None;

pub trait FontButtonExt: 'static {
    #[cfg_attr(feature = "v3_22", deprecated)]
    fn get_font_name(&self) -> Option<GString>;

    fn get_show_size(&self) -> bool;

    fn get_show_style(&self) -> bool;

    fn get_title(&self) -> Option<GString>;

    fn get_use_font(&self) -> bool;

    fn get_use_size(&self) -> bool;

    #[cfg_attr(feature = "v3_22", deprecated)]
    fn set_font_name(&self, fontname: &str) -> bool;

    fn set_show_size(&self, show_size: bool);

    fn set_show_style(&self, show_style: bool);

    fn set_title(&self, title: &str);

    fn set_use_font(&self, use_font: bool);

    fn set_use_size(&self, use_size: bool);

    fn connect_font_set<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_22", deprecated)]
    fn connect_property_font_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_font_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FontButton>> FontButtonExt for O {
    fn get_font_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_font_button_get_font_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_show_size(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_font_button_get_show_size(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_show_style(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_font_button_get_show_style(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_title(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_font_button_get_title(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_use_font(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_font_button_get_use_font(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_use_size(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_font_button_get_use_size(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_font_name(&self, fontname: &str) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_font_button_set_font_name(
                self.as_ref().to_glib_none().0,
                fontname.to_glib_none().0,
            ))
        }
    }

    fn set_show_size(&self, show_size: bool) {
        unsafe {
            gtk_sys::gtk_font_button_set_show_size(
                self.as_ref().to_glib_none().0,
                show_size.to_glib(),
            );
        }
    }

    fn set_show_style(&self, show_style: bool) {
        unsafe {
            gtk_sys::gtk_font_button_set_show_style(
                self.as_ref().to_glib_none().0,
                show_style.to_glib(),
            );
        }
    }

    fn set_title(&self, title: &str) {
        unsafe {
            gtk_sys::gtk_font_button_set_title(
                self.as_ref().to_glib_none().0,
                title.to_glib_none().0,
            );
        }
    }

    fn set_use_font(&self, use_font: bool) {
        unsafe {
            gtk_sys::gtk_font_button_set_use_font(
                self.as_ref().to_glib_none().0,
                use_font.to_glib(),
            );
        }
    }

    fn set_use_size(&self, use_size: bool) {
        unsafe {
            gtk_sys::gtk_font_button_set_use_size(
                self.as_ref().to_glib_none().0,
                use_size.to_glib(),
            );
        }
    }

    fn connect_font_set<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn font_set_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFontButton,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FontButton>,
        {
            let f: &F = &*(f as *const F);
            f(&FontButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"font-set\0".as_ptr() as *const _,
                Some(*(&font_set_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_font_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFontButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FontButton>,
        {
            let f: &F = &*(f as *const F);
            f(&FontButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::font-name\0".as_ptr() as *const _,
                Some(*(&notify_font_name_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_show_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_size_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFontButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FontButton>,
        {
            let f: &F = &*(f as *const F);
            f(&FontButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-size\0".as_ptr() as *const _,
                Some(*(&notify_show_size_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_show_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_style_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFontButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FontButton>,
        {
            let f: &F = &*(f as *const F);
            f(&FontButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-style\0".as_ptr() as *const _,
                Some(*(&notify_show_style_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFontButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FontButton>,
        {
            let f: &F = &*(f as *const F);
            f(&FontButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title\0".as_ptr() as *const _,
                Some(*(&notify_title_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_use_font_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_font_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFontButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FontButton>,
        {
            let f: &F = &*(f as *const F);
            f(&FontButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-font\0".as_ptr() as *const _,
                Some(*(&notify_use_font_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_use_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_size_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFontButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FontButton>,
        {
            let f: &F = &*(f as *const F);
            f(&FontButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-size\0".as_ptr() as *const _,
                Some(*(&notify_use_size_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FontButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FontButton")
    }
}
