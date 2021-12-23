use gdnative::api::{Camera, InputEvent, InputEventMouse, InputEventMouseMotion, Spatial};
use gdnative::prelude::*;
use std::cmp::{max, min};

#[derive(NativeClass)]
#[inherit(Camera)]
pub struct PlayerCamera {}

#[methods]
impl PlayerCamera {
    fn process_camera_movement(&mut self, event: Ref<InputEventMouseMotion>, owner: &Camera) {
        let rotation_helper = unsafe { 
            owner.get_node_as::<Spatial>("../").unwrap() 
        };

        let event = unsafe { 
            event.assume_safe() 
        };

        let relative_rotation = event.relative();

        let rotation_y = f64::from(relative_rotation.y) * 0.02;
        rotation_helper.rotate_x(rotation_y.to_radians());
    }

    #[export]
    fn _ready(&mut self, _owner: &Camera) {
        Input::godot_singleton().set_mouse_mode(Input::MOUSE_MODE_CAPTURED);
    }

    fn new(_owner: &Camera) -> Self {
        PlayerCamera {}
    }

    #[export]
    fn _input(&mut self, owner: &Camera, event: Ref<InputEvent>) {
        if let Ok(event) = event.try_cast::<InputEventMouseMotion>() {
            self.process_camera_movement(event, owner);
        }
    }
}
