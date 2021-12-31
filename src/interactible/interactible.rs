use gdnative::api::PhysicsBody;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(PhysicsBody)]
pub struct Interactible {
    //We should find a way to define texts as multilang, probably by providing trans file and fetch it by key
    //we will assume usabe of trans keys for now
    #[property]
    name_label: String,

    #[property]
    inspect_label: String,

    #[property]
    interaction_label: String,

    #[property]
    mob_type: String,
}

#[methods]
impl Interactible {
    fn new(_owner: &PhysicsBody) -> Self {
        Interactible {
            name_label: String::from(""),
            inspect_label: String::from(""),
            interaction_label: String::from(""),
            mob_type: String::from(""),
        }
    }

    #[export]
    fn on_hover(&self, _owner: &PhysicsBody) {
        godot_print!("{}", self.name_label);
    }

    #[export]
    fn on_inspect(&self, _owner: &PhysicsBody) {
        godot_print!("{}", self.inspect_label);
    }

    #[export]
    fn on_open(&self, _owner: &PhysicsBody) {
        godot_print!("{}", "Im trying to open myself");
    }

    #[export]
    fn interact(&self, owner: &PhysicsBody) {
        unsafe {
            owner.assume_unique().queue_free();
        }
    }
}
