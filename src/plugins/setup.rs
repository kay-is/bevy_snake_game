use crate::{events, resources, systems::spawn_snake};
use bevy::prelude::*;

pub(crate) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(resources::SnakeSegments::default())
        .insert_resource(resources::LastTailPosition::default())
        .add_event::<events::GrowthEvent>()
        .add_event::<events::GameOverEvent>()
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_snake.system()));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
