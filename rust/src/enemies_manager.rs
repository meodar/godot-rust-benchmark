use godot::{
    classes::{Input, Timer},
    prelude::*,
};

use crate::{enemy::Enemy, random_table};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct EnemiesManager {
    #[export]
    top_enemy_scene: OnEditor<Gd<PackedScene>>,
    #[export]
    side_enemy_scene: OnEditor<Gd<PackedScene>>,
    #[export]
    spawn_timer: OnEditor<Gd<Timer>>,
    #[export]
    benchmark_timer: OnEditor<Gd<Timer>>,
    #[init(val = 10)]
    benchmark_value: u32,
    benchmark_sum: f32,
    spawn_cursor: usize,
    side_cursor: usize,
    tick_cursor: usize,
    workload_cursor: usize,

    base: Base<Node>,
}

impl EnemiesManager {
    pub fn spawn_enemy(&mut self, is_top: bool) {
        if let Some(mut parent) = self.base().get_parent() {
            let mut instance: Gd<Enemy> = if is_top {
                self.top_enemy_scene.instantiate_as::<Enemy>()
            } else {
                self.side_enemy_scene.instantiate_as::<Enemy>()
            };

            let mut tf = instance.get_transform();

            let table = random_table::table();
            if is_top {
                let x = table.spawn_x(self.spawn_cursor);
                tf.origin = Vector3::new(x, 0.0, -800.0);
            } else {
                let y = table.spawn_y(self.spawn_cursor);
                tf.origin = Vector3::new(0.0, y, -800.0);
            }
            self.spawn_cursor = self.spawn_cursor.wrapping_add(1);

            instance.set_transform(tf);

            parent.add_child(&instance);
        }
    }
}

#[godot_api]
impl EnemiesManager {
    fn on_timeout(&mut self) {
        let is_top = random_table::table().spawn_top(self.side_cursor);
        self.side_cursor = self.side_cursor.wrapping_add(1);
        self.spawn_enemy(is_top);
    }

    fn on_benchmark_timeout(&mut self) {
        for _ in 0..self.benchmark_value {
            self.spawn_enemy(true);
        }
        for _ in 0..self.benchmark_value {
            self.spawn_enemy(false);
        }
        self.benchmark_value += 10;
    }
}

#[godot_api]
impl INode for EnemiesManager {
    fn ready(&mut self) {
        self.spawn_timer
            .signals()
            .timeout()
            .connect_other(self, Self::on_timeout);

        self.benchmark_timer
            .signals()
            .timeout()
            .connect_other(self, Self::on_benchmark_timeout);
    }

    fn physics_process(&mut self, _delta: f64) {
        let input = Input::singleton();
        if input.is_action_just_pressed("benchmark_spawn") {
            for _ in 0..50 {
                self.spawn_enemy(true);
            }
            for _ in 0..50 {
                self.spawn_enemy(false);
            }
        }

        let table = random_table::table();
        let heavy_tick = table.heavy_tick(self.tick_cursor);
        self.tick_cursor = self.tick_cursor.wrapping_add(1);

        if heavy_tick {
            let mut acc = 0.0_f32;
            let mut local_data = Vec::new();
            let mut cursor = self.workload_cursor;
            for _ in 0..500 {
                let i = table.workload_500(cursor);
                cursor = cursor.wrapping_add(1);
                let x = (i as f32 * std::f32::consts::PI / 180.0).sin() * ((i as f32 + 1.0).sqrt());
                acc += x.powf(1.5);
                local_data.push(acc);
            }
            self.workload_cursor = cursor;

            self.benchmark_sum = acc;
        }
    }
}
