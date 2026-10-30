use godot::builtin::{PackedByteArray, PackedFloat32Array, PackedInt32Array};
use godot::classes::RandomNumberGenerator;
use godot::obj::NewGd;
use godot::prelude::{GodotClass, godot_api};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};

pub const SEED: u64 = 1_234_567_890;

const WORKLOAD_10K_LEN: usize = 4096;
const WORKLOAD_500_LEN: usize = 2048;
const SPAWN_LEN: usize = 1024;
const MOVEMENT_MOD_LEN: usize = 4096;
const MOVEMENT_KIND_LEN: usize = 1024;

const _: () = assert!(
    WORKLOAD_10K_LEN.is_power_of_two()
        && WORKLOAD_500_LEN.is_power_of_two()
        && SPAWN_LEN.is_power_of_two()
        && MOVEMENT_MOD_LEN.is_power_of_two()
        && MOVEMENT_KIND_LEN.is_power_of_two()
);

pub struct RandomTable {
    workload_10k: Box<[i32; WORKLOAD_10K_LEN]>,
    workload_500: Box<[i32; WORKLOAD_500_LEN]>,
    spawn_x: Box<[f32; SPAWN_LEN]>,
    spawn_y: Box<[f32; SPAWN_LEN]>,
    spawn_top: Box<[bool; SPAWN_LEN]>,
    heavy_tick: Box<[bool; SPAWN_LEN]>,
    movement_mod: Box<[f32; MOVEMENT_MOD_LEN]>,
    movement_kind: Box<[u8; MOVEMENT_KIND_LEN]>,
}

#[allow(dead_code)]
impl RandomTable {
    #[inline]
    pub fn workload_10k(&self, index: usize) -> i32 {
        self.workload_10k[index & (WORKLOAD_10K_LEN - 1)]
    }

    #[inline]
    pub fn workload_500(&self, index: usize) -> i32 {
        self.workload_500[index & (WORKLOAD_500_LEN - 1)]
    }

    #[inline]
    pub fn spawn_x(&self, index: usize) -> f32 {
        self.spawn_x[index & (SPAWN_LEN - 1)]
    }

    #[inline]
    pub fn spawn_y(&self, index: usize) -> f32 {
        self.spawn_y[index & (SPAWN_LEN - 1)]
    }

    #[inline]
    pub fn spawn_top(&self, index: usize) -> bool {
        self.spawn_top[index & (SPAWN_LEN - 1)]
    }

    #[inline]
    pub fn heavy_tick(&self, index: usize) -> bool {
        self.heavy_tick[index & (SPAWN_LEN - 1)]
    }

    #[inline]
    pub fn movement_mod(&self, index: usize) -> f32 {
        self.movement_mod[index & (MOVEMENT_MOD_LEN - 1)]
    }

    #[inline]
    pub fn movement_kind(&self, index: usize) -> u8 {
        self.movement_kind[index & (MOVEMENT_KIND_LEN - 1)]
    }
}

static TABLE: OnceLock<RandomTable> = OnceLock::new();

static SPAWN_INDEX: AtomicUsize = AtomicUsize::new(0);

pub fn table() -> &'static RandomTable {
    TABLE.get_or_init(build)
}

pub fn warm_up() {
    let _ = table();
}

pub fn next_spawn_index() -> usize {
    SPAWN_INDEX.fetch_add(1, Ordering::Relaxed)
}

fn build() -> RandomTable {
    let mut rng = RandomNumberGenerator::new_gd();
    rng.set_seed(SEED);

    RandomTable {
        workload_10k: build_table(|| rng.randi_range(1, 9_999)),
        workload_500: build_table(|| rng.randi_range(1, 499)),
        spawn_x: build_table(|| rng.randf_range(-1000.0, 1000.0)),
        spawn_y: build_table(|| rng.randf_range(-800.0, 800.0)),
        spawn_top: build_table(|| rng.randf() < 0.5),
        heavy_tick: build_table(|| rng.randf() < 0.1),
        movement_mod: build_table(|| rng.randf_range(0.2, 2.0)),
        movement_kind: build_table(|| rng.randi_range(0, 4) as u8),
    }
}

fn build_table<T, const N: usize>(mut next: impl FnMut() -> T) -> Box<[T; N]> {
    let mut values = Vec::with_capacity(N);
    values.resize_with(N, &mut next);
    values
        .into_boxed_slice()
        .try_into()
        .unwrap_or_else(|_| unreachable!("the vector is filled with exactly N elements"))
}

#[derive(GodotClass)]
#[class(no_init, base=Object)]
pub struct BenchmarkRandomTable;

#[godot_api]
impl BenchmarkRandomTable {
    #[func]
    fn warm_up() {
        self::warm_up();
    }

    #[func]
    fn next_spawn_index() -> i64 {
        self::next_spawn_index() as i64
    }

    #[func]
    fn workload_10k() -> PackedInt32Array {
        PackedInt32Array::from(&table().workload_10k[..])
    }

    #[func]
    fn workload_500() -> PackedInt32Array {
        PackedInt32Array::from(&table().workload_500[..])
    }

    #[func]
    fn spawn_x() -> PackedFloat32Array {
        PackedFloat32Array::from(&table().spawn_x[..])
    }

    #[func]
    fn spawn_y() -> PackedFloat32Array {
        PackedFloat32Array::from(&table().spawn_y[..])
    }

    #[func]
    fn spawn_top() -> PackedByteArray {
        table()
            .spawn_top
            .iter()
            .map(|&flag| u8::from(flag))
            .collect()
    }

    #[func]
    fn heavy_tick() -> PackedByteArray {
        table()
            .heavy_tick
            .iter()
            .map(|&flag| u8::from(flag))
            .collect()
    }

    #[func]
    fn movement_mod() -> PackedFloat32Array {
        PackedFloat32Array::from(&table().movement_mod[..])
    }

    #[func]
    fn movement_kind() -> PackedByteArray {
        PackedByteArray::from(&table().movement_kind[..])
    }
}
