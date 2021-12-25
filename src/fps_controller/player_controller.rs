use gdnative::api::{Camera, KinematicBody};
use gdnative::prelude::*;

const GRAVITY: f32 = -24.8;
const MAX_SPEED: f32 = 20.0;
const ACCEL: f32 = 4.5;
const DEACCEL: f32 = 16.0;
const MAX_SLOPE_ANGLE: f64 = 40.0;

#[derive(NativeClass)]
#[inherit(KinematicBody)]
pub struct PlayerController {
    velocity: Vector3,
    direction: Vector3,
}

#[methods]
impl PlayerController {
    fn new(_owner: &KinematicBody) -> Self {
        PlayerController {
            velocity: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody, delta: f64) {
        self.process_input(owner);
        self.process_movement(owner, delta);
    }

    //Direction needs to be reset on every processing
    fn reset_direction(&mut self) {
        self.direction = Vector3::new(0.0, 0.0, 0.0);
    }

    fn get_camera_direction(&self, owner: &KinematicBody) -> Transform {
        unsafe {
            owner
                .get_node_as::<Camera>("CameraRotationHelper/Camera")
                .unwrap()
                .global_transform()
        }
    }

    fn process_input(&mut self, owner: &KinematicBody) {
        self.reset_direction();
        let camera_direction = self.get_camera_direction(owner);

        let mut movement_vector = Vector2::new(0.0, 0.0);

        let input = Input::godot_singleton();

        if Input::is_action_pressed(input, "move_forward", false) {
            movement_vector.y -= 1.0;
        }
        if Input::is_action_pressed(input, "move_backward", false) {
            movement_vector.y += 1.0;
        }

        if Input::is_action_pressed(input, "move_left", false) {
            movement_vector.x -= 1.0;
        }

        if Input::is_action_pressed(input, "move_right", false) {
            movement_vector.x += 1.0;
        }

        if movement_vector.length() != 0.0 {
            movement_vector = movement_vector.normalized();
        }

        self.direction += camera_direction.basis.c() * movement_vector.y;
        self.direction += camera_direction.basis.a() * movement_vector.x;
    }

    //I remember there beign lot of redudant stuff.. so refactor this later on
    fn process_movement(&mut self, owner: &KinematicBody, delta: f64) {
        self.direction.y = 0.0;

        if self.direction.length() != 0.0 {
            self.direction = self.direction.normalized();
        }

        self.velocity.y += (delta * GRAVITY as f64) as f32;
        let mut horizontal_velocity = self.velocity;
        horizontal_velocity.y = 0.0;

        let mut target = self.direction;
        target *= MAX_SPEED;

        let mut acceleration = ACCEL;
        if self.direction.dot(horizontal_velocity) <= 0.0 {
            acceleration = DEACCEL
        }

        horizontal_velocity =
            horizontal_velocity.linear_interpolate(target, (acceleration as f64 * delta) as f32);
        self.velocity.x = horizontal_velocity.x;
        self.velocity.z = horizontal_velocity.z;

        self.velocity = owner.move_and_slide(
            self.velocity,
            Vector3::new(0.0, 1.0, 0.0),
            true,
            4,
            MAX_SLOPE_ANGLE.to_radians(),
            true,
        );
    }
}
