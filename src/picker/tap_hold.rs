use cascade::cascade;
use gtk::{
    glib::{self, clone, subclass::Signal},
    pango,
    prelude::*,
    subclass::prelude::*,
};
use once_cell::sync::Lazy;
use std::cell::{Cell, RefCell};

use super::{picker_group_box::PickerGroupBox, PickerKey, SCANCODE_LABELS};
use backend::{DerefCell, Keycode, Mods};

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
pub struct TapHoldInner {
    mods: Cell<Mods>,
    keycode: RefCell<Option<String>>,
    mod_buttons: DerefCell<Vec<PickerKey>>,
}

#[glib::object_subclass]
impl ObjectSubclass for TapHoldInner {
    const NAME: &'static str = "S76KeyboardTapHold";
    type ParentType = gtk::Box;
    type Type = TapHold;
}

impl ObjectImpl for TapHoldInner {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder(
                "selected",
                &[Keycode::static_type().into()],
                glib::Type::UNIT.into(),
            )
            .build()]
        });
        SIGNALS.as_ref()
    }

    fn constructed(&self, widget: &Self::Type) {
        self.parent_constructed(widget);

        let layout = backend::Layout::from_board("system76/launch_1").unwrap();

        let picker_group_box = cascade! {
            PickerGroupBox::new("basics");
            ..connect_key_pressed(clone!(@weak widget => move |name| {
                *widget.inner().keycode.borrow_mut() = Some(name);
                widget.update();
            }));
            // Correct?
            ..set_key_visibility(|name| layout.scancode_from_name(&Keycode::Basic(Mods::empty(), name.to_string())).map_or(false, |code| code <= 0xff));
        };

        let modifier_button_box = cascade! {
            gtk::Box::new(gtk::Orientation::Horizontal, 0);
        };
        let mut mod_buttons = Vec::new();
        for i in MODIFIERS {
            let label = SCANCODE_LABELS.get(*i).unwrap();
            let mod_ = Mods::from_mod_str(*i).unwrap();
            let button = cascade! {
                PickerKey::new(i, label, 2);
                ..connect_clicked(clone!(@weak widget => move |_| {
                    // XXX shift self
                    // XXX mark selected
                    widget.inner().mods.set(mod_);
                    widget.update();
                }));
            };
            modifier_button_box.add(&button);
            mod_buttons.push(button);
        }
        self.mod_buttons.set(mod_buttons);

        let layer_button_box = cascade! {
            gtk::Box::new(gtk::Orientation::Horizontal, 0);
        };
        for i in LAYERS {
            let label = SCANCODE_LABELS.get(*i).unwrap();
            layer_button_box.add(&PickerKey::new(i, label, 2));
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

    fn update(&self) {
        let mods = self.inner().mods.get();
        if !mods.is_empty() {
            let keycode = self.inner().keycode.borrow();
            let keycode = keycode.as_deref().unwrap_or("NONE");
            self.emit_by_name::<()>("selected", &[&Keycode::MT(mods, keycode.to_string())]);
        }
    }

    // XXX naming vs set_selected
    pub fn connect_selected<F: Fn(Keycode) + 'static>(&self, cb: F) -> glib::SignalHandlerId {
        self.connect_local("selected", false, move |values| {
            cb(values[1].get::<Keycode>().unwrap());
            None
        })
    }

    pub(crate) fn set_selected(&self, scancode_names: Vec<Keycode>) {
        // XXX how to handle > 1?
        let (mods, keycode) = if scancode_names.len() == 1 {
            if let Keycode::MT(mods, keycode) = &scancode_names[0] {
                (mods.clone(), Some(keycode))
            } else {
                (Mods::empty(), None)
            }
        } else {
            (Mods::empty(), None)
        };

        for i in self.inner().mod_buttons.iter() {
            // XXX left vs right
            let mod_ = Mods::from_mod_str(i.name()).unwrap();
            i.set_selected(
                mods.contains(mod_) && (mods.contains(Mods::RIGHT) == mod_.contains(Mods::RIGHT)),
            );
        }

        // set_selected on group box
    }
}
