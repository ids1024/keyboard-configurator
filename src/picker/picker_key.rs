use cascade::cascade;
use gtk::prelude::*;
use std::rc::Rc;

pub(super) struct PickerKey {
    /// Symbolic name of the key
    pub(super) name: String,
    // GTK button
    pub(super) gtk: gtk::Button,
}

impl PickerKey {
    pub(super) fn new(name: String, text: String, width: i32) -> Rc<Self> {
        let label = cascade! {
            gtk::Label::new(Some(&text));
            ..set_line_wrap(true);
            ..set_max_width_chars(1);
            ..set_margin_start(5);
            ..set_margin_end(5);
            ..set_justify(gtk::Justification::Center);
        };

        let button = cascade! {
            gtk::Button::new();
            ..style_context().add_class("picker-key");
            ..set_size_request(48 * width, 48);
            ..add(&label);
            ..show_all();
        };

        Rc::new(Self { name, gtk: button })
    }
}
