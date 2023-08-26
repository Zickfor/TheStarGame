use std::collections::HashMap;
use std::f32::consts::PI;
use hecs::World;
use macroquad::camera::{Camera2D, set_camera};
use macroquad::math::Rect;
use macroquad::window::{clear_background, screen_height, screen_width};

use macroquad::texture::{draw_texture_ex, DrawTextureParams};
use macroquad::color::{BLACK, WHITE};
use macroquad::miniquad::debug;
use macroquad::prelude::Texture2D;
use crate::components::{ObjectType, Position, Rotation, UnderControl};

pub fn camera_set_on_position(pos: &Position) {
    let camera = Camera2D::from_display_rect(Rect {
        x: pos.x - screen_width() / 2.0,
        y: pos.y - screen_height() / 2.0,
        w: screen_width(),
        h: screen_height(),
    });
    set_camera(&camera);
}

pub fn set_camera_on_ship(world: &mut World) {
    for (_id, (pos, _under_control)) in world.query_mut::<(&Position, &UnderControl)>() {
        camera_set_on_position(pos)
    }
}

pub fn texture_drawer(world: &mut World, textures: &HashMap<String, Texture2D>) {
    clear_background(BLACK);
    set_camera_on_ship(world);
    let mut c: usize = 0;
    for (id, (pos, rot, object_type)) in world.query_mut::<(&Position, &Rotation, &ObjectType)>() {
        let params = DrawTextureParams {
            dest_size: None,
            source: None,
            rotation: rot.0 + PI / 2.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };

        match object_type {
            ObjectType::Ship => {
                let x = pos.x - textures.get("ship.png").unwrap().width() / 2.0;
                let y = pos.y - textures.get("ship.png").unwrap().height() / 2.0;
                draw_texture_ex(textures.get("ship.png").unwrap(), x, y, WHITE, params)
            }
            ObjectType::Asteroid => {
                let x = pos.x - textures.get("asteroid.png").unwrap().width() / 2.0;
                let y = pos.y - textures.get("asteroid.png").unwrap().height() / 2.0;
                draw_texture_ex(textures.get("asteroid.png").unwrap(), pos.x, pos.y, WHITE, params)
            }
            ObjectType::Bullet => {
                let x = pos.x - textures.get("bullet.png").unwrap().width() / 2.0;
                let y = pos.y - textures.get("bullet.png").unwrap().height() / 2.0;
                draw_texture_ex(textures.get("bullet.png").unwrap(), pos.x, pos.y, WHITE, params)
            }
        }
        c += 1;
    }
    debug!("texture_drawer handled {} entities", c)
}