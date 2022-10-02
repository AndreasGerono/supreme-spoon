#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

mod colissions;
mod end_turn;
mod entities_renderer;
mod map_renders;
mod player_input;
mod random_move;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_renders::map_render_system())
        .add_system(entities_renderer::render_entities_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(colissions::colissions_system())
        .flush()
        .add_system(map_renders::map_render_system())
        .add_system(entities_renderer::render_entities_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_monsters_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move::random_move_system())
        .flush()
        .add_system(colissions::colissions_system())
        .add_system(map_renders::map_render_system())
        .add_system(entities_renderer::render_entities_system())
        .add_system(end_turn::end_turn_system())
        .build()
}
