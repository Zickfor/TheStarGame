use hecs;
use hecs::*;

use macroquad::prelude::*;
use macroquad::rand::gen_range;
use components::{MaxSpeed, ObjectType, Position, Rotation, Size, Speed, Textures, UnderControl, VelocityPower};


mod systems;
mod components;

fn create_ship(world: &mut World) -> Entity {
    return world.spawn((
        Position { x: 0.0, y: 0.0 },
        Speed(0.0),
        Rotation(0.0),
        UnderControl,
        MaxSpeed(500.0),
        VelocityPower(0.0),
        ObjectType::Ship
    ));
}

fn create_asteroid(world: &mut World) -> Entity {
    return world.spawn((Position { x: gen_range(-100.0, 100.0), y: gen_range(-100.0, 100.0) },
                        Rotation(gen_range(-3.14, 3.14)),
                        Speed(gen_range(-20.0, 200.0)),
                        Size(gen_range(1.0, 20.0)),
                        ObjectType::Asteroid));
}

fn create_asteroids(world: &mut World, n: u32) {
    for _ in 0..n {
        create_asteroid(world);
    }
}

#[macroquad::main("The Star Game")]
async fn main() {
    let mut world = World::new();
    let ship = create_ship(&mut world);
    create_asteroids(&mut world, 100);

    let textures = Textures {
        ship: load_texture("assets/ship.png").await.unwrap(),
        asteroid: load_texture("assets/asteroid.png").await.unwrap(),
        bullet: load_texture("assets/bullet.png").await.unwrap(),
    };

    loop {
        systems::systems(&mut world, ship, textures);
        next_frame().await
    }
}