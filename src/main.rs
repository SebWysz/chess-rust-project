#![deny(clippy::all)]

mod bevy_fns;
use bevy_fns::array::structs::{InCheck, WhiteMove};
use bevy_fns::{setup_board, spawn_camera, mouse_click_system};

use bevy::prelude::*; 

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(setup_board)
        .add_system(mouse_click_system)
        .insert_resource(WhiteMove(true))
        .insert_resource(InCheck {
            black: false,
            white: false,
        })
        .run();
}

// also have to somehow check that the king is not in check --- TBD