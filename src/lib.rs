use std::collections::HashMap;
use hecs::World;
use macroquad::prelude::{load_texture, Texture2D};
use macroquad::window::next_frame;
use crate::systems::{controls, physics, startup};

mod systems;
mod components;
mod utils;

pub struct Game {
    server: Server,
    client: Client,
    state: GameState,
}

impl Game {
    pub async fn default() -> Self {
        Self {
            server: Server::default(),
            client: Client::default(),
            state: GameState::Play,
        }
    }
    pub async fn startup(&mut self) {
        self.server.startup();
        self.client.startup().await;
    }

    pub async fn game_loop(&mut self) {
        loop {
            self.server.tick();
            self.client.draw(&mut self.server.world).await;
        }
    }
}

#[derive(Default)]
struct Server {
    world: World,
    ticks: u64,
}

impl Server {
    fn load() -> Self {
        todo!()
    }

    fn startup(&mut self) {
        startup::create_asteroids(&mut self.world, 100);
        startup::create_ship(&mut self.world);
    }

    fn tick(&mut self) {
        controls::handle_movement_controls(&mut self.world);
        match controls::handle_fire_controls(&mut self.world) {
            None => {}
            Some(bullets) => { self.world.spawn(bullets); }
        };
        physics::speed_update(&mut self.world);
        physics::position_update(&mut self.world);
        self.ticks += 1;
    }
}

#[derive(Default, Clone)]
struct Client {
    textures: HashMap<String, Texture2D>,
}

impl Client {
    async fn startup(&mut self) {
        self.load_textures().await
    }

    async fn load_textures(&mut self) {
        let paths = std::fs::read_dir("assets/").unwrap();
        for path in paths {
            let p = path.unwrap();
            self.textures.insert(p.file_name().to_str().unwrap().parse().unwrap(), load_texture(p.path().to_str().unwrap()).await.unwrap());
        }
    }

    async fn draw(&mut self, world: &mut World) {
        systems::render::texture_drawer(world, &self.textures);
        next_frame().await;
    }
}

enum GameState {
    Pause,
    Play,
}
