use godot::prelude::*;

#[derive(GodotConvert, Var, Export, Default, Clone, Copy)]
#[godot(via = GString)]
pub enum EnemyType {
    #[default]
    Top,
    Side,
}
