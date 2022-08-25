use cascade::cascade;
use gtk::{
    glib::{self, clone},
    pango,
    prelude::*,
};

use backend::{Keycode, Mods};
use super::picker_group_box::PickerGroupBox;

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

pub fn tap_hold_box() -> gtk::Box {
    // XXX
    let layout = backend::Layout::from_board("system76/launch_1").unwrap();

    let picker_group_box = cascade! {
        PickerGroupBox::new("basics");
        ..connect_key_pressed(move |name| {
        });
        // Correct?
        ..set_key_visibility(|name| layout.scancode_from_name(&Keycode::Basic(Mods::empty(), Some(name.to_string()))).map_or(false, |code| code <= 0xff));
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

    cascade! {
        gtk::Box::new(gtk::Orientation::Vertical, 0);
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
        // - populate from picker.json, only "basic"
    }
}
