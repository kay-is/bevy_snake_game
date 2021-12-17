use bevy::prelude::*;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) enum SnakeMovement {
    Input,
    Movement,
    Eating,
    Growth,
}
