use super::spawn_segment;
use crate::{
    events::GrowthEvent,
    resources::{LastTailPosition, Materials, SnakeSegments},
};
use bevy::prelude::*;

pub(crate) fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
    materials: Res<Materials>,
) {
    if growth_reader.iter().next().is_some() {
        segments.0.push(spawn_segment(
            commands,
            &materials.segment_material,
            last_tail_position.0.unwrap(),
        ));
    }
}
