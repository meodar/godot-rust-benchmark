use godot::{
    classes::{Area3D, IArea3D, Marker3D, Timer},
    prelude::*,
};

use crate::{bullet::Bullet, enemy_movement::EnemyMovement, enemy_type::EnemyType, random_table};

#[derive(GodotClass)]
#[class(base=Area3D)]
pub struct Enemy {
    #[export]
    speed: f32,
    #[export]
    enemy_type: EnemyType,
    #[export]
    enemy_movement: EnemyMovement,
    #[export]
    movement_mod: Vector3,
    #[export]
    shooting_timer: OnEditor<Gd<Timer>>,
    #[export]
    bullet: OnEditor<Gd<PackedScene>>,
    #[export]
    shooting_point: OnEditor<Gd<Marker3D>>,
    time_elapsed: f64,
    workload_cursor: usize,
    base: Base<Area3D>,
}

#[godot_api]
impl Enemy {
    #[signal]
    pub fn bullet_hit();

    #[func]
    pub fn on_bullet_hit(&mut self) {
        self.base_mut().queue_free();
    }

    fn on_shoot_timeout(&mut self) {
        let global_tf = self.shooting_point.get_global_transform();
        let bullet_scene = self.bullet.clone();
        if let Some(mut parent) = self.base_mut().get_parent() {
            let mut bullet = bullet_scene.instantiate_as::<Bullet>();
            bullet.set_global_transform(global_tf);
            parent.add_child(&bullet);
        }
    }
}

#[godot_api]
impl IArea3D for Enemy {
    fn init(base: Base<Area3D>) -> Enemy {
        let spawn_index = random_table::next_spawn_index();
        let table = random_table::table();
        Enemy {
            base,
            speed: 200.0,
            enemy_type: EnemyType::default(),
            enemy_movement: EnemyMovement::from_table(spawn_index),
            movement_mod: Vector3::new(
                table.movement_mod(spawn_index * 3),
                table.movement_mod(spawn_index * 3 + 1),
                table.movement_mod(spawn_index * 3 + 2),
            ),
            time_elapsed: 0.0,
            shooting_point: OnEditor::default(),
            shooting_timer: OnEditor::default(),
            bullet: OnEditor::default(),
            workload_cursor: spawn_index,
        }
    }

    fn ready(&mut self) {
        self.shooting_timer
            .signals()
            .timeout()
            .connect_other(self, Self::on_shoot_timeout);

        self.signals()
            .bullet_hit()
            .connect_self(Self::on_bullet_hit);
    }

    fn physics_process(&mut self, delta: f64) {
        self.time_elapsed += delta;
        let t = self.time_elapsed as f32;
        let dz = self.speed * delta as f32;

        let table = random_table::table();
        let mut cursor = self.workload_cursor;
        let mut sum = 0.0_f64;
        for _ in 0..7_000 {
            let i = table.workload_10k(cursor);
            cursor = cursor.wrapping_add(1);
            sum += (i as f64 * std::f64::consts::PI).sqrt() * (i as f64 * 0.01).cos();
        }
        self.workload_cursor = cursor;

        if sum.is_sign_negative() {
            println!("weird");
        }

        let mut tf = self.base().get_transform();
        let a = self.movement_mod.x;
        let b = self.movement_mod.y;
        let c = self.movement_mod.z;
        match self.enemy_movement {
            EnemyMovement::Lin => {
                tf.origin.z += dz;
            }
            EnemyMovement::Cos => {
                let phase = a * (b * t + c).cos() * 500.0;
                match self.enemy_type {
                    EnemyType::Top => tf.origin.x = phase,
                    EnemyType::Side => tf.origin.y = phase,
                }
                tf.origin.z += dz;
            }
            EnemyMovement::Sin => {
                let phase = a * (b * t + c).sin() * 500.0;
                match self.enemy_type {
                    EnemyType::Top => tf.origin.x = phase,
                    EnemyType::Side => tf.origin.y = phase,
                }
                tf.origin.z += dz;
            }
            EnemyMovement::Log => {
                let new_z = tf.origin.z + dz;
                let v = a * ((new_z.abs().max(1.0) + c).ln()) * 200.0;
                match self.enemy_type {
                    EnemyType::Top => tf.origin.x = v,
                    EnemyType::Side => tf.origin.y = v,
                }
                tf.origin.z = new_z;
            }
            EnemyMovement::Sqrt => {
                let new_z = tf.origin.z + dz;
                let v = a * (new_z.abs().sqrt() + c) * 100.0;
                match self.enemy_type {
                    EnemyType::Top => tf.origin.x = v,
                    EnemyType::Side => tf.origin.y = v,
                }
                tf.origin.z = new_z;
            }
        }

        self.base_mut().set_transform(tf);

        if tf.origin.z >= 800.0 {
            self.base().clone().queue_free();
        }
    }
}
