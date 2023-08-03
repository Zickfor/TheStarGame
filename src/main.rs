use std::collections::HashMap;
use hecs;
use hecs::*;

use macroquad::prelude::*;

mod systems;
mod components;
mod utils;

#[macroquad::main("The Star Game")]
async fn main() {
    let mut world = World::new();

    systems::systems_startup(&mut world);

    let mut textures: HashMap<&str, Texture2D> = HashMap::new();
    textures.insert("ship", load_texture("assets/ship.png").await.unwrap());
    textures.insert("asteroid", load_texture("assets/asteroid.png").await.unwrap());
    textures.insert("bullet", load_texture("assets/bullet.png").await.unwrap());

    loop {
        systems::systems_cycled(&mut world, &textures);
        next_frame().await
    }
}