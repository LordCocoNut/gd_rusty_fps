mod fps_controller;

use crate::fps_controller::player_camera::PlayerCamera;
use gdnative::prelude::*;


fn init(handle: InitHandle) {
    handle.add_class::<PlayerCamera>();
}

godot_init!(init);