use cascade::cascade;
use gtk::{
    glib::{self, clone},
    pango,
    prelude::*,
    subclass::prelude::*,
};

use super::picker_group_box::PickerGroupBox;
use backend::{Keycode, Mods};

// XXX translate
static MODIFIERS: &[&str] = &[
    "Left Shift",
    "Left Control",
    "Left Super",
    "Left Alt",
    "Right Shift",
    "Right Control",
    "Right Super",
    "Right Alt",
];
static LAYERS: &[&str] = &[
    "Access Layer 1",
    "Access Layer 2",
    "Access Layer 3",
    "Access Layer 4",
];

#[derive(Default)]
pub struct TapHoldInner;

#[glib::object_subclass]
impl ObjectSubclass for TapHoldInner {
    const NAME: &'static str = "S76KeyboardTapHold";
    type ParentType = gtk::Box;
    type Type = TapHold;
}

impl ObjectImpl for TapHoldInner {
    fn constructed(&self, widget: &Self::Type) {
        self.parent_constructed(widget);

        let layout = backend::Layout::from_board("system76/launch_1").unwrap();

        let picker_group_box = cascade! {
            PickerGroupBox::new("basics");
            ..connect_key_pressed(move |name| {
            });
            // Correct?
            ..set_key_visibility(|name| layout.scancode_from_name(&Keycode::Basic(Mods::empty(), name.to_string())).map_or(false, |code| code <= 0xff));
        };

        let modifier_button_box = cascade! {
            gtk::Box::new(gtk::Orientation::Horizontal, 0);
        };
        for i in MODIFIERS {
            modifier_button_box.add(&gtk::Button::with_label(i));
        }

        let layer_button_box = cascade! {
            gtk::Box::new(gtk::Orientation::Horizontal, 0);
        };
        for i in LAYERS {
            layer_button_box.add(&gtk::Button::with_label(i));
        }

        // TODO: select monifier/layer; multiple select; when both are selected, set keycode

        cascade! {
            widget;
            ..set_orientation(gtk::Orientation::Vertical);
            ..add(&cascade! {
                gtk::Label::new(Some("1. Select action(s) to use when the key is held."));
                ..set_attributes(Some(&cascade! {
                    pango::AttrList::new();
                    ..insert(pango::AttrInt::new_weight(pango::Weight::Bold));
                }));
                ..set_halign(gtk::Align::Start);
            });
            ..add(&modifier_button_box);
            ..add(&layer_button_box);
            ..add(&cascade! {
                gtk::Label::new(Some("2. Select an action to use when the key is tapped."));
                ..set_attributes(Some(&cascade! {
                    pango::AttrList::new();
                    ..insert(pango::AttrInt::new_weight(pango::Weight::Bold));
                }));
                ..set_halign(gtk::Align::Start);
            });
            ..add(&picker_group_box);
        };
    }
}

impl BoxImpl for TapHoldInner {}
impl WidgetImpl for TapHoldInner {}
impl ContainerImpl for TapHoldInner {}

glib::wrapper! {
    pub struct TapHold(ObjectSubclass<TapHoldInner>)
        @extends gtk::Box, gtk::Container, gtk::Widget, @implements gtk::Orientable;
}

impl TapHold {
    pub fn new() -> Self {
        glib::Object::new(&[]).unwrap()
    }

    fn inner(&self) -> &TapHoldInner {
        TapHoldInner::from_instance(self)
    }
}
