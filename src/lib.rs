mod fps_controller;

use crate::fps_controller::player_camera::PlayerCamera;
use crate::fps_controller::player_controller::PlayerController;
use gdnative::prelude::*;


fn init(handle: InitHandle) {
    handle.add_class::<PlayerController>();
    handle.add_class::<PlayerCamera>();
}

godot_init!(init);