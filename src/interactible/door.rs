use gdnative::api::PhysicsBody;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(PhysicsBody)]
pub struct Door {
    #[property]
    name_label: String,
    
    #[property(no_editor)]
    mob_type: String,

    #[property]
    interaction_label: String,
}

#[methods]
impl Door {
    fn new(_owner: &PhysicsBody) -> Self {
        Door {
            name_label: String::from(""),
            mob_type: String::from("Openable"),
            interaction_label: String::from(""),
        }
    }

    #[export]
    fn on_hover(&self, _owner: &PhysicsBody) {
        godot_print!("{}", "Open with e key");
    }

    #[export]
    fn interact(&self, owner: &PhysicsBody) {
        self.open(owner);
    }

    #[export]
    fn open(&self, _owner: &PhysicsBody){
        godot_print!("{:?}", _owner);
    }

    #[export]
    fn close(&self, _owner: &PhysicsBody) {

    }
}


