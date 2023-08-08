use std::collections::HashMap;
use hecs::World;
use macroquad::camera::{Camera2D, set_camera};
use macroquad::math::Rect;
use macroquad::window::{clear_background, screen_height, screen_width};

use macroquad::texture::{draw_texture_ex, DrawTextureParams};
use macroquad::color::{BLACK, WHITE};
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
    for (_id, (pos, rot, object_type)) in world.query_mut::<(&Position, &Rotation, &ObjectType)>() {
        let params = DrawTextureParams {
            dest_size: None,
            source: None,
            rotation: rot.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };

        match object_type {
            ObjectType::Ship => {
                draw_texture_ex(textures.get("ship.png").unwrap(), pos.x, pos.y, WHITE, params)
            },
            ObjectType::Asteroid => {
                draw_texture_ex(textures.get("asteroid.png").unwrap(), pos.x, pos.y, WHITE, params)
            },
            ObjectType::Bullet => {
                draw_texture_ex(textures.get("bullet.png").unwrap(), pos.x, pos.y, WHITE, params)
            }
        }
    }
}