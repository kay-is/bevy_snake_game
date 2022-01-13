use crate::{
    components::{Position, Size, SnakeSegment},
    constants::SNAKE_SEGMENT_COLOR,
};
use bevy::prelude::*;

pub(crate) fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}
