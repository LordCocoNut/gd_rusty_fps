use gdnative::api::RigidBody;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(RigidBody)]
pub struct Pickupable {
    #[property]
    name_label: String,

    #[property]
    interaction_label: String,

    #[property(no_editor)]
    mob_type: String,
}

#[methods]
impl Pickupable {
    fn new(_owner: &RigidBody) -> Self {
        Pickupable {
            name_label: String::from(""),
            mob_type: String::from("Pickupable"),
            interaction_label: String::from(""),
        }
    }

    #[export]
    fn on_hover(&self, _owner: &RigidBody) {
        godot_print!("{}", "Pickup with e key");
    }

    #[export]
    fn interact(&self, owner: &RigidBody) {
        let mut string = String::from("Picked up: ");
        string.push_str(owner.get("name_label").to_string().as_str());
        godot_print!("{}", string);
        unsafe {owner.get_parent().unwrap().assume_safe()}.queue_free();
    }
}


