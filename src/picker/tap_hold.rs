use cascade::cascade;
use gtk::{
    glib::{self, clone},
    pango,
    prelude::*,
    subclass::prelude::*,
};

use super::{picker_group_box::PickerGroupBox, SCANCODE_LABELS};
use backend::{Keycode, Mods};

static MODIFIERS: &[&str] = &[
    "LEFT_SHIFT",
    "LEFT_CTRL",
    "LEFT_SUPER",
    "LEFT_ALT",
    "RIGHT_SHIFT",
    "RIGHT_CTRL",
    "RIGHT_SUPER",
    "RIGHT_ALT",
];
static LAYERS: &[&str] = &["LAYER_ACCESS_1", "FN", "LAYER_ACCESS_3", "LAYER_ACCESS_4"];

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
            let label = SCANCODE_LABELS.get(*i).unwrap();
            modifier_button_box.add(&gtk::Button::with_label(label));
        }

        let layer_button_box = cascade! {
            gtk::Box::new(gtk::Orientation::Horizontal, 0);
        };
        for i in LAYERS {
            let label = SCANCODE_LABELS.get(*i).unwrap();
            layer_button_box.add(&gtk::Button::with_label(label));
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
