use cascade::cascade;
use gtk::{glib, prelude::*, subclass::prelude::*};

use backend::DerefCell;

#[derive(Default)]
pub struct PickerKeyInner {
    label: DerefCell<gtk::Label>,
    name: DerefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for PickerKeyInner {
    const NAME: &'static str = "S76PickerKey";
    type ParentType = gtk::Button;
    type Type = PickerKey;
}

impl ObjectImpl for PickerKeyInner {
    fn constructed(&self, widget: &Self::Type) {
        let label = cascade! {
            gtk::Label::new(None);
            ..set_line_wrap(true);
            ..set_max_width_chars(1);
            ..set_margin_start(5);
            ..set_margin_end(5);
            ..set_justify(gtk::Justification::Center);
        };

        cascade! {
            widget;
            ..style_context().add_class("picker-key");
            ..add(&label);
            ..show_all();
        };

        self.label.set(label);
    }
}
impl WidgetImpl for PickerKeyInner {}
impl ContainerImpl for PickerKeyInner {}
impl BinImpl for PickerKeyInner {}
impl ButtonImpl for PickerKeyInner {}

glib::wrapper! {
    pub struct PickerKey(ObjectSubclass<PickerKeyInner>)
        @extends gtk::Button, gtk::Bin, gtk::Container, gtk::Widget, @implements gtk::Orientable;
}

impl PickerKey {
    pub fn new(name: String, text: String, width: i32) -> Self {
        let widget: Self = glib::Object::new(&[]).unwrap();
        widget.inner().name.set(name);
        widget.inner().label.set_label(&text);
        widget.set_size_request(48 * width, 48);
        widget
    }

    fn inner(&self) -> &PickerKeyInner {
        PickerKeyInner::from_instance(self)
    }

    /// Symbolic name of the key
    pub fn name(&self) -> &str {
        &*self.inner().name
    }
}
