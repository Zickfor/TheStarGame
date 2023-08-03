use std::f32::consts::PI;

use hecs::{Entity, World};
use macroquad::input::{is_key_down, is_key_pressed, KeyCode};

use crate::components::{MaxSpeed, Position, Rotation, Speed, UnderControl, VelocityPower};

pub fn handle_movement_controls(world: &mut World, ship: Entity) {
    let (_pos, rot, speed, _under_control, velocity_power, max_speed) = world.query_one_mut::<(&Position, &mut Rotation, &mut Speed, &UnderControl, &mut VelocityPower, &MaxSpeed)>(ship).unwrap();

    if is_key_pressed(KeyCode::W) {
        if velocity_power.0 < 1.0 {
            velocity_power.0 += 0.1;
        }
        speed.0 = max_speed.0 * velocity_power.0;
    }
    if is_key_pressed(KeyCode::S) {
        if velocity_power.0 >= -0.1 {
            velocity_power.0 -= 0.1;
        }
        speed.0 = max_speed.0 * velocity_power.0;
    }
    if is_key_down(KeyCode::A) {
        rot.0 += PI / 180.0;
    }
    if is_key_down(KeyCode::D) {
        rot.0 -= PI / 180.0;
    }
}

pub fn handle_fire_controls(world: &mut World, ship_pos_and_rot: (&Position, &Rotation)) {
    let pos = ship_pos_and_rot.0;
    let rot = ship_pos_and_rot.1;
    if is_key_pressed(KeyCode::Q) {
        world.spawn((Position { x: pos.x-16.0, y: pos.y-8.0 }, Rotation(rot.0)));
    }
}