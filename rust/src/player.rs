use godot::{
    classes::{Area3D, IArea3D, Input, Marker3D},
    prelude::*,
};

use crate::{bullet::Bullet, camera_mode::CameraMode};

#[derive(GodotClass)]
#[class(init, base=Area3D)]
pub struct Player {
    #[export]
    #[init(val = 600.0)]
    speed: f32,
    #[export]
    #[init(val = 1000)]
    health: u32,
    #[export]
    bullet: OnEditor<Gd<PackedScene>>,
    #[export]
    shooting_point: OnEditor<Gd<Marker3D>>,
    #[init(val = CameraMode::default())]
    camera_mode: CameraMode,
    base: Base<Area3D>,
}

impl Player {
    pub fn change_camera_mode(&mut self, camera_mode: CameraMode) {
        self.camera_mode = camera_mode;
    }
}

#[godot_api]
impl Player {
    #[signal]
    pub fn take_damage(damage: u32);

    #[func]
    pub fn on_take_damage(&mut self, damage: u32) {
        self.health = self.health.saturating_sub(damage);
        if self.health == 0 {
            self.base_mut().queue_free();
        }
    }
}

#[godot_api]
impl IArea3D for Player {
    fn ready(&mut self) {
        self.signals()
            .take_damage()
            .connect_self(Self::on_take_damage);
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();
        let mut tf = self.base().get_transform();

        match self.camera_mode {
            CameraMode::Top { .. } => {
                if input.is_action_pressed("ui_up") {
                    tf.origin.z -= self.speed * delta as f32;
                }
                if input.is_action_pressed("ui_down") {
                    tf.origin.z += self.speed * delta as f32;
                }
                if input.is_action_pressed("ui_left") {
                    tf.origin.x -= self.speed * delta as f32;
                }
                if input.is_action_pressed("ui_right") {
                    tf.origin.x += self.speed * delta as f32;
                }
            }
            CameraMode::Side { .. } => {
                if input.is_action_pressed("ui_up") {
                    tf.origin.y += self.speed * delta as f32;
                }
                if input.is_action_pressed("ui_down") {
                    tf.origin.y -= self.speed * delta as f32;
                }
                if input.is_action_pressed("ui_left") {
                    tf.origin.z += self.speed * delta as f32;
                }
                if input.is_action_pressed("ui_right") {
                    tf.origin.z -= self.speed * delta as f32;
                }
            }
        }

        if input.is_action_just_pressed("fire") {
            let global_tf = self.shooting_point.get_global_transform();
            let bullet_scene = self.bullet.clone();
            if let Some(mut parent) = self.base_mut().get_parent() {
                let mut bullet = bullet_scene.instantiate_as::<Bullet>();
                bullet.set_global_transform(global_tf);
                parent.add_child(&bullet);
            }
        }

        self.base_mut().set_transform(tf);
    }
}
