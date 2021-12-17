use crate::components::Position;
use bevy::prelude::*;

pub(crate) struct Materials {
    pub head_material: Handle<ColorMaterial>,
    pub segment_material: Handle<ColorMaterial>,
    pub food_material: Handle<ColorMaterial>,
}

#[derive(Default)]
pub(crate) struct SnakeSegments(pub Vec<Entity>);

#[derive(Default)]
pub(crate) struct LastTailPosition(pub Option<Position>);
