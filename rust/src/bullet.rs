use godot::{
    classes::{Area3D, IArea3D},
    prelude::*,
};

use crate::{enemy::Enemy, player::Player};

#[derive(GodotClass)]
#[class(init, base=Area3D)]
pub struct Bullet {
    #[init(val = 1000.0)]
    #[export]
    speed: f32,
    #[init(val = 300)]
    #[export]
    damage: u32,
    base: Base<Area3D>,
}

#[godot_api]
impl Bullet {
    #[func]
    fn on_body_entered(&mut self, area: Gd<Area3D>) {
        if let Ok(player) = area.clone().try_cast::<Player>() {
            player.signals().take_damage().emit(self.damage);
            self.base_mut().queue_free();
        } else if let Ok(enemy) = area.try_cast::<Enemy>() {
            enemy.signals().bullet_hit().emit();
            self.base_mut().queue_free();
        }
    }
}

#[godot_api]
impl IArea3D for Bullet {
    fn ready(&mut self) {
        self.signals()
            .area_entered()
            .connect_self(Self::on_body_entered);
    }

    fn physics_process(&mut self, delta: f64) {
        let mut tf = self.base().get_transform();
        let forward = tf.basis.xform_inv(Vector3::new(0.0, 0.0, 1.0));
        tf.origin += forward.normalized() * self.speed * delta as f32;
        self.base_mut().set_transform(tf);

        if tf.origin.z.abs() > 1000.0 {
            self.base_mut().queue_free();
        }
    }
}
