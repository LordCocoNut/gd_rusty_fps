use crate::interactible::interactible::Interactible;
use gdnative::api::{InputEvent, RayCast, Spatial, StaticBody};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(RayCast)]
pub struct PlayerCameraFocus;

#[methods]
impl PlayerCameraFocus {
    fn new(_owner: &RayCast) -> Self {
        PlayerCameraFocus {}
    }

    #[export]
    fn _physics_process(&self, owner: &RayCast) {
        godot_print!("{:?}", unsafe {
            owner.get_collider().unwrap().try_cast::<Interactible>()
        });
    }
}
