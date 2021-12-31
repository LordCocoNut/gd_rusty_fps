use gdnative::api::RayCast;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(RayCast)]
pub struct PlayerCameraFocus {}

#[methods]
impl PlayerCameraFocus {
    fn new(_owner: &RayCast) -> Self {
        PlayerCameraFocus {}
    }

    fn handle_item_focus(&self, owner: &RayCast) {
        //If raycast is colliding
        if owner.is_colliding() {
            let collider = unsafe { owner.get_collider().unwrap().assume_safe() };
            let input = Input::godot_singleton();

            //Lets handle interaction
            if Input::is_action_just_pressed(input, "interact", false) {
                match collider.get("mob_type").to_string().as_str() {
                    "Pickupable" => unsafe {
                        collider.call("interact", &[]);
                    },
                    "Openable" => unsafe {
                        collider.call("on_open", &[]);
                    },
                    _ => godot_print!("I dont know what to do with this"),
                }
            }

            //Lets handle inspect of item
            if Input::is_action_just_pressed(input, "inspect", false) {
                if collider.has_method("on_inspect") {
                    unsafe {
                        collider.call("on_inspect", &[]);
                    }
                }
            }
        }
    }

    #[export]
    fn _physics_process(&self, owner: &RayCast, delta: f64) {
        self.handle_item_focus(owner);
    }
}
