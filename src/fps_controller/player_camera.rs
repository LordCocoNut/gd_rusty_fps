
use super::player_camera_focus::PlayerCameraFocus;
use gdnative::api::{
    Camera, InputEvent, InputEventMouseMotion, KinematicBody, Spatial
};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Camera)]
pub struct PlayerCamera {}

#[methods]
impl PlayerCamera {
    fn new(_owner: &Camera) -> Self {
        PlayerCamera {}
    }

    fn process_camera_movement(&mut self, event: Ref<InputEventMouseMotion>, camera: &Camera) {        
        let rotation_helper = unsafe { camera.get_node_as::<Spatial>("../").unwrap() };
        let event = unsafe { event.assume_safe() };
        let relative_rotation = event.relative();

        let player_body = unsafe {
            camera
                .get_parent()
                .unwrap()
                .assume_safe()
                .get_parent()
                .unwrap()
                .assume_safe()
                .cast::<KinematicBody>()
                .unwrap()
        };
        

        //Build relative rotation
        let rotation_y = (-f32::from(relative_rotation.y) * 0.04).to_radians();
        let rotation_x = (-f64::from(relative_rotation.x) * 0.04).to_radians();

        //Rotate camera
        rotation_helper.rotate_x(f64::from(rotation_y));

        //Rotate body
        player_body.rotate_y(rotation_x);

        //Clamp the view range (up, down)
        let mut camera_rotation = rotation_helper.rotation_degrees();
        camera_rotation.x = camera_rotation.x.clamp(-80.0, 80.0);
        rotation_helper.set_rotation_degrees(camera_rotation);
    }

    #[export]
    fn _ready(&mut self, _owner: &Camera) {
        Input::godot_singleton().set_mouse_mode(Input::MOUSE_MODE_CAPTURED);
    }


    #[export]
    fn _input(&mut self, owner: &Camera, event: Ref<InputEvent>) {
        if let Ok(event) = event.try_cast::<InputEventMouseMotion>() {
            self.process_camera_movement(event, owner);
        }
    }

}
