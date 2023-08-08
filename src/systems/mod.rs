use std::collections::HashMap;
use hecs::{Entity, World};
use macroquad::prelude::Texture2D;

pub mod startup;
pub mod physics;
pub mod render;
pub mod controls;

pub fn systems_startup(world: &mut World) -> Entity {
    startup::create_asteroids(world, 100);
    startup::create_ship(world)
}

pub fn systems_cycled(world: &mut World, textures: &HashMap<String, Texture2D>) {
    controls::handle_movement_controls(world);
    physics::system_motion(world);
    render::texture_drawer(world, textures);
}