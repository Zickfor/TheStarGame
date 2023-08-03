use std::collections::HashMap;
use hecs::{Entity, World};
use macroquad::prelude::Texture2D;
use crate::components::{Position, Rotation};

pub mod physics;
pub mod render;
pub mod controls;

pub fn systems(world: &mut World, ship: Entity, textures: &HashMap<&str, Texture2D>) {
    controls::handle_movement_controls(world, ship);
    controls::handle_fire_controls(&mut Default::default(), world.query_one::<(&Position, &Rotation)>(ship).unwrap().get().unwrap());

    physics::system_motion(world);

    render::texture_drawer(world, textures);
}