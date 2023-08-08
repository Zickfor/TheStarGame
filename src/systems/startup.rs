use hecs::{Entity, World};
use macroquad::rand::gen_range;
use macroquad::math::u32;
use crate::components::{MaxSpeed, ObjectType, Position, Rotation, Size, Speed, UnderControl, VelocityPower};

pub fn create_ship(world: &mut World) -> Entity {
    world.spawn((
        Position { x: 0.0, y: 0.0 },
        Speed(0.0),
        Rotation(0.0),
        UnderControl,
        MaxSpeed(500.0),
        VelocityPower(0.0),
        ObjectType::Ship
    ))
}

fn create_asteroid(world: &mut World) -> Entity {
    world.spawn((Position { x: gen_range(-100.0, 100.0), y: gen_range(-100.0, 100.0) },
                        Rotation(gen_range(-std::f32::consts::PI, std::f32::consts::PI)),
                        Speed(gen_range(-20.0, 200.0)),
                        Size(gen_range(1.0, 20.0)),
                        ObjectType::Asteroid))
}

pub fn create_asteroids(world: &mut World, n: u32) {
    for _ in 0..n {
        create_asteroid(world);
    }
}
