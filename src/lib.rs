mod fps_controller;
mod interactible;
mod inventory_system;

use crate::fps_controller::player_camera::PlayerCamera;
use crate::fps_controller::player_controller::PlayerController;
use crate::fps_controller::player_camera_focus::PlayerCameraFocus;
use crate::interactible::{door::Door, pickupable::Pickupable};
use crate::inventory_system::inventory::Inventory;
use gdnative::prelude::*;


fn init(handle: InitHandle) {
    handle.add_class::<PlayerController>();
    handle.add_class::<PlayerCamera>();
    handle.add_class::<Door>();
    handle.add_class::<Pickupable>();
    handle.add_class::<PlayerCameraFocus>();
    handle.add_class::<Inventory>();
}

godot_init!(init);