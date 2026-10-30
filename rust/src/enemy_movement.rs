use godot::prelude::*;

use crate::random_table;

#[derive(GodotConvert, Var, Export, Default, Clone, Copy)]
#[godot(via = GString)]
pub enum EnemyMovement {
    #[default]
    Lin,
    Cos,
    Sin,
    Log,
    Sqrt,
}

impl EnemyMovement {
    pub fn from_table(spawn_index: usize) -> Self {
        match random_table::table().movement_kind(spawn_index) {
            0 => EnemyMovement::Lin,
            1 => EnemyMovement::Cos,
            2 => EnemyMovement::Sin,
            3 => EnemyMovement::Log,
            _ => EnemyMovement::Sqrt,
        }
    }
}
