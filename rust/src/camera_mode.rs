use godot::prelude::*;
use godot_core::meta::{ByValue, shape::GodotShape};

#[derive(Clone, Debug)]
pub enum CameraMode {
    Top { last_side_pos: f32 },
    Side { last_top_pos: f32 },
}

impl Default for CameraMode {
    fn default() -> Self {
        Self::Top { last_side_pos: 0.0 }
    }
}

impl GodotConvert for CameraMode {
    type Via = VarDictionary;

    fn godot_shape() -> GodotShape {
        <Self::Via as GodotConvert>::godot_shape()
    }
}

impl FromGodot for CameraMode {
    fn try_from_godot(via: VarDictionary) -> Result<Self, ConvertError> {
        let mode_variant = via
            .get(&"mode".to_variant())
            .ok_or_else(|| ConvertError::new("ConvertError: missing key 'mode'"))?;
        let mode: String = mode_variant.try_to()?;

        let pos_variant = via
            .get(&"pos".to_variant())
            .ok_or_else(|| ConvertError::new("ConvertError: missing key 'pos'"))?;
        let pos_f64: f64 = pos_variant.try_to()?;
        let pos_f32 = pos_f64 as f32;

        match mode.as_str() {
            "top" => Ok(CameraMode::Top {
                last_side_pos: pos_f32,
            }),
            "side" => Ok(CameraMode::Side {
                last_top_pos: pos_f32,
            }),
            other => Err(ConvertError::new(&format!(
                "ConvertError: unknown CameraMode variant `{}`",
                other
            ))),
        }
    }
}

impl ToGodot for CameraMode {
    type Pass = ByValue;

    fn to_godot(&self) -> Self::Via {
        let mut dict = VarDictionary::new();
        match self {
            CameraMode::Top { last_side_pos } => {
                let _ = dict.insert(&"mode".to_variant(), &"top".to_variant());
                let _ = dict.insert(&"pos".to_variant(), &last_side_pos.to_variant());
            }
            CameraMode::Side { last_top_pos } => {
                let _ = dict.insert(&"mode".to_variant(), &"side".to_variant());
                let _ = dict.insert(&"pos".to_variant(), &last_top_pos.to_variant());
            }
        }
        dict
    }
}
