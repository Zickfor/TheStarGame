use hecs::{Entity, World};
use crate::components::{Position, Rotation, Textures};

pub mod physics;
pub mod render;
pub mod controls;

pub fn systems(world: &mut World, ship: Entity, textures: Textures) {
    controls::handle_movement_controls(world, ship);
    controls::handle_fire_controls(&mut Default::default(), world.query_one::<(&Position, &Rotation)>(ship).unwrap().get().unwrap());

    physics::system_motion(world);

    render::texture_drawer(world, textures);
}