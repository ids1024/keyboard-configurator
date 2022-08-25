use cascade::cascade;
use futures::{prelude::*, stream::FuturesUnordered};
use gtk::{
    glib::{self, clone},
    prelude::*,
    subclass::prelude::*,
};
use once_cell::sync::Lazy;
use std::{cell::RefCell, collections::HashMap};

use crate::Keyboard;
use backend::{DerefCell, Keycode, Mods};

mod picker_group;
mod picker_group_box;
mod picker_json;
mod picker_key;
mod tap_hold;

use picker_group_box::PickerGroupBox;
use picker_json::picker_json;
use picker_key::PickerKey;

pub static SCANCODE_LABELS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut labels = HashMap::new();
    for group in picker_json() {
        for key in group.keys {
            labels.insert(key.keysym, key.label);
        }
    }
    labels
});

#[derive(Default)]
pub struct PickerInner {
    group_boxes: DerefCell<Vec<PickerGroupBox>>,
    keyboard: RefCell<Option<Keyboard>>,
}

#[glib::object_subclass]
impl ObjectSubclass for PickerInner {
    const NAME: &'static str = "S76KeyboardPicker";
    type ParentType = gtk::Box;
    type Type = Picker;
}

impl ObjectImpl for PickerInner {
    fn constructed(&self, picker: &Picker) {
        self.parent_constructed(picker);

        let basics_group_box = cascade! {
            PickerGroupBox::new("basics");
            ..connect_key_pressed(clone!(@weak picker => move |name| {
                picker.key_pressed(name)
            }));
        };

        let extras_group_box = cascade! {
            PickerGroupBox::new("extras");
            ..connect_key_pressed(clone!(@weak picker => move |name| {
                picker.key_pressed(name)
            }));
        };

        // XXX translate
        let stack = cascade! {
            gtk::Stack::new();
            ..add_titled(&basics_group_box, "basics", "Basics");
            ..add_titled(&extras_group_box, "extras", "Extras");
            ..add_titled(&tap_hold::tap_hold_box(), "tap-hold", "Tap-Hold");
        };

        let stack_switcher = cascade! {
            gtk::StackSwitcher::new();
            ..set_stack(Some(&stack));
        };

        cascade! {
            picker;
            ..set_orientation(gtk::Orientation::Vertical);
            ..add(&stack_switcher);
            ..add(&stack);
            ..show_all();
        };

        self.group_boxes
            .set(vec![basics_group_box, extras_group_box]);
    }
}

impl BoxImpl for PickerInner {}

impl WidgetImpl for PickerInner {}

impl ContainerImpl for PickerInner {}

glib::wrapper! {
    pub struct Picker(ObjectSubclass<PickerInner>)
        @extends gtk::Box, gtk::Container, gtk::Widget, @implements gtk::Orientable;
}

impl Picker {
    pub fn new() -> Self {
        glib::Object::new(&[]).unwrap()
    }

    fn inner(&self) -> &PickerInner {
        PickerInner::from_instance(self)
    }

    pub(crate) fn set_keyboard(&self, keyboard: Option<Keyboard>) {
        if let Some(old_kb) = &*self.inner().keyboard.borrow() {
            old_kb.set_picker(None);
        }

        if let Some(kb) = &keyboard {
            // Check that scancode is available for the keyboard
            for group_box in self.inner().group_boxes.iter() {
                group_box.set_key_visibility(|name| kb.has_scancode(&Keycode::Basic(Mods::empty(), Some(name.to_string()))));
            }
            kb.set_picker(Some(&self));
        }

        *self.inner().keyboard.borrow_mut() = keyboard;
    }

    pub(crate) fn set_selected(&self, scancode_names: Vec<Keycode>) {
        for group_box in self.inner().group_boxes.iter() {
            group_box.set_selected(scancode_names.clone());
        }
    }

    fn key_pressed(&self, name: String) {
        let kb = match self.inner().keyboard.borrow().clone() {
            Some(kb) => kb,
            None => {
                return;
            }
        };
        let layer = kb.layer();

        if let Some(layer) = layer {
            let futures = FuturesUnordered::new();
            for i in kb.selected().iter() {
                let i = *i;
                futures.push(clone!(@strong kb, @strong name => async move {
                    kb.keymap_set(i, layer, &Keycode::Basic(Mods::empty(), Some(name))).await;
                }));
            }
            glib::MainContext::default().spawn_local(async { futures.collect::<()>().await });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use backend::{layouts, Layout};
    use std::collections::HashSet;

    #[test]
    fn picker_has_keys() {
        let mut missing = HashSet::new();
        for i in layouts() {
            let layout = Layout::from_board(i).unwrap();
            for j in layout.default.map.values().flatten() {
                if SCANCODE_LABELS.keys().find(|x| x == &j).is_none() {
                    missing.insert(j.to_owned());
                }
            }
        }
        assert_eq!(missing, HashSet::new());
    }
}
