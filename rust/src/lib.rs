use godot::prelude::*;

mod bullet;
mod camera_mode;
mod enemies_manager;
mod enemy;
mod enemy_movement;
mod enemy_type;
mod land;
mod perf_monitor;
mod player;
mod random_table;

struct BenchmarkRustRsExt;

#[gdextension]
unsafe impl ExtensionLibrary for BenchmarkRustRsExt {}
