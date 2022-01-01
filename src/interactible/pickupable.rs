use gdnative::api::PhysicsBody;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(PhysicsBody)]
pub struct Pickupable {
    #[property]
    name_label: String,

    #[property]
    interaction_label: String,

    mob_type: String,
}

#[methods]
impl Pickupable {
    fn new(_owner: &PhysicsBody) -> Self {
        Pickupable {
            name_label: String::from(""),
            mob_type: String::from("Pickupable"),
            interaction_label: String::from(""),
        }
    }

    #[export]
    fn on_hover(&self, _owner: &PhysicsBody) {
        godot_print!("{}", "Pickup with e key");
    }

    #[export]
    fn interact(&self, owner: &PhysicsBody) {
        let mut string = String::from("Picked up: ");
        string.push_str(owner.get("name_label").to_string().as_str());
        godot_print!("{}", string);
    }
}


