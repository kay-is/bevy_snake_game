mod components;
mod constants;
mod events;
mod labels;
mod plugins;
mod resources;
mod systems;

use bevy::core::FixedTimestep;
use bevy::prelude::*;
use plugins::*;
use systems::*;

fn main() {
    let timed_150ms = SystemSet::new().with_run_criteria(FixedTimestep::step(0.150));
    let timed_1sec = SystemSet::new().with_run_criteria(FixedTimestep::step(1.0));

    App::build()
        .add_plugin(SetupPlugin)
        .add_system(
            snake_movement_input
                .system()
                .label(labels::SnakeMovement::Input)
                .before(labels::SnakeMovement::Movement),
        )
        .add_system_set(
            timed_150ms
                .with_system(
                    snake_movement
                        .system()
                        .label(labels::SnakeMovement::Movement),
                )
                .with_system(
                    snake_eating
                        .system()
                        .label(labels::SnakeMovement::Eating)
                        .after(labels::SnakeMovement::Movement),
                )
                .with_system(
                    snake_growth
                        .system()
                        .label(labels::SnakeMovement::Growth)
                        .after(labels::SnakeMovement::Eating),
                ),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation.system())
                .with_system(size_scaling.system()),
        )
        .add_system_set(timed_1sec.with_system(food_spawner.system()))
        .add_system(game_over.system().after(labels::SnakeMovement::Movement))
        .add_plugins(DefaultPlugins)
        .run();
}
