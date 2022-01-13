use super::spawn_snake;
use crate::{
    components::{Food, SnakeSegment},
    events::GameOverEvent,
    resources::SnakeSegments,
};
use bevy::prelude::*;

pub(crate) fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, segments_res);
    }
}
