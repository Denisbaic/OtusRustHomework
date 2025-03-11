use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::MouseButton;
use bevy::reflect::Reflect;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::{prelude::InputMap, Actionlike};

// https://github.com/Leafwing-Studios/leafwing-input-manager/tree/main/examples

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Actionlike, Reflect, Default)]
pub(crate) enum CharacterAction {
    #[default]
    #[actionlike(Axis)]
    Move,
    #[actionlike(Axis)]
    Zooming,
    ChooseMagic1,
    ChooseMagic2,
    ChooseMagic3,
    SpawnMagic,
}

impl CharacterAction {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert_axis(Self::Move, VirtualAxis::ad());

        input_map.insert_axis(Self::Zooming, MouseScrollAxis::Y);

        input_map.insert(Self::ChooseMagic1, KeyCode::Digit1);
        input_map.insert(Self::ChooseMagic2, KeyCode::Digit2);
        input_map.insert(Self::ChooseMagic3, KeyCode::Digit3);

        input_map.insert(Self::SpawnMagic, MouseButton::Left);

        input_map
    }
}
