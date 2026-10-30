use godot::{
    classes::{Camera3D, Input},
    prelude::*,
};

use crate::{camera_mode::CameraMode, player::Player, random_table};

#[derive(GodotClass)]
#[class(init, base=Node)]
struct Land {
    #[export]
    land: OnEditor<Gd<Node3D>>,
    #[export]
    second_land: OnEditor<Gd<Node3D>>,
    #[export]
    camera: OnEditor<Gd<Camera3D>>,
    #[export]
    player: OnEditor<Gd<Player>>,
    #[init(val = CameraMode::default())]
    camera_mode: CameraMode,

    base: Base<Node>,
}

#[godot_api]
impl Land {
    #[signal]
    pub fn camera_view_changed(camera_mode: CameraMode);
}

#[godot_api]
impl INode for Land {
    fn ready(&mut self) {
        random_table::warm_up();

        let player = self.player.clone();
        self.signals()
            .camera_view_changed()
            .connect_other(&player, Player::change_camera_mode);
    }

    fn physics_process(&mut self, delta: f64) {
        Self::update_terrain(&mut self.land, delta);
        Self::update_terrain(&mut self.second_land, delta);

        let input = Input::singleton();
        if input.is_action_just_pressed("camera_change") {
            match self.camera_mode {
                CameraMode::Top { last_side_pos } => {
                    let mut tf = self.camera.get_transform();
                    tf.basis = tf
                        .basis
                        .rotated(Vector3::FORWARD, std::f32::consts::FRAC_PI_2);
                    tf.basis = tf
                        .basis
                        .rotated(Vector3::RIGHT, std::f32::consts::FRAC_PI_2);
                    tf.origin += Vector3::new(1000.0, -1000.0, 0.0);
                    self.camera.set_transform(tf);

                    let mut ptf = self.player.get_transform();
                    let last_top_x = ptf.origin.x;
                    ptf.origin = Vector3::new(0.0, last_side_pos, ptf.origin.z);
                    self.player.set_transform(ptf);

                    self.camera_mode = CameraMode::Side {
                        last_top_pos: last_top_x,
                    };
                }
                CameraMode::Side { last_top_pos } => {
                    let mut tf = self.camera.get_transform();
                    tf.basis = tf
                        .basis
                        .rotated(Vector3::FORWARD, -std::f32::consts::FRAC_PI_2);
                    tf.basis = tf.basis.rotated(Vector3::UP, -std::f32::consts::FRAC_PI_2);
                    tf.origin += Vector3::new(-1000.0, 1000.0, 0.0);
                    self.camera.set_transform(tf);

                    let mut ptf = self.player.get_transform();
                    let last_side_y = ptf.origin.y;
                    ptf.origin = Vector3::new(last_top_pos, 0.0, ptf.origin.z);
                    self.player.set_transform(ptf);

                    self.camera_mode = CameraMode::Top {
                        last_side_pos: last_side_y,
                    };
                }
            }
            let mode = self.camera_mode.clone();
            self.signals().camera_view_changed().emit_tuple((mode,));
        }
    }
}

impl Land {
    fn update_terrain(terrain: &mut Gd<Node3D>, delta: f64) {
        let mut transform = terrain.get_transform();
        transform.origin.z += 80.0 * delta as f32;
        if transform.origin.z >= 32000.0 {
            transform.origin.z -= 64000.0;
        }
        terrain.set_transform(transform);
    }
}
