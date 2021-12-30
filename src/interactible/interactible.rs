use gdnative::prelude::*;
use gdnative::api::{StaticBody};

#[derive(NativeClass)]
#[inherit(StaticBody)]
pub struct Interactible {
    #[property]
    name: String,
}

#[methods]
impl Interactible {
    fn new(_owner: &StaticBody) -> Self {
        Interactible {
            name: String::from("Interactible"),
        }
    }

    #[export]
    pub fn on_hover(&self, _owner: &StaticBody) {
        godot_print!("Im interactible!");
    }
}