use super::spawn_segment;
use crate::{
    components::{Direction, Position, Size, SnakeHead, SnakeSegment},
    constants::SNAKE_HEAD_COLOR,
    resources::SnakeSegments,
};
use bevy::prelude::*;

pub(crate) fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    segments.0 = vec![
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_HEAD_COLOR,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(SnakeSegment)
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(commands, Position { x: 3, y: 2 }),
    ];
}
