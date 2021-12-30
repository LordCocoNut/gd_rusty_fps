use gdnative::api::{RayCast};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(RayCast)]
pub struct PlayerCameraFocus {
    
}

#[methods]
impl PlayerCameraFocus {
    fn new(_owner: &RayCast) -> Self {
        PlayerCameraFocus{}
    }

    fn handle_item_focus(&self, owner: &RayCast) {
        //If raycast is colliding
        if owner.is_colliding() {
             if unsafe { owner.get_collider().unwrap().assume_safe().has_method("on_hover")} {
                unsafe { owner.get_collider().unwrap().assume_safe().call("on_hover", &[])};
             }
        }
    }

    #[export]
    fn _physics_process(&self, owner: &RayCast, delta: f64) {
        self.handle_item_focus(owner);
    }
}

