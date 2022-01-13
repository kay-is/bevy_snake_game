use crate::components::Position;
use bevy::prelude::*;

#[derive(Default)]
pub(crate) struct SnakeSegments(pub Vec<Entity>);

#[derive(Default)]
pub(crate) struct LastTailPosition(pub Option<Position>);
