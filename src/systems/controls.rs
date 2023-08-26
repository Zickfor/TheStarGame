use std::f32::consts::PI;

use hecs::World;
use macroquad::input::{is_key_down, is_key_pressed, KeyCode};
use macroquad::logging::debug;

use crate::components::{MaxSpeed, ObjectType, Position, Rotation, Speed, UnderControl, VelocityPower};

pub fn handle_movement_controls(world: &mut World) {
    let mut c: usize = 0;
    for (_id, (rot, speed, _under_control, velocity_power, max_speed)) in world.query_mut::<(&mut Rotation, &mut Speed, &UnderControl, &mut VelocityPower, &MaxSpeed)>().into_iter() {
        if is_key_pressed(KeyCode::W) {
            velocity_power.speed_up()
        }
        if is_key_pressed(KeyCode::S) {
            velocity_power.speed_down()
        }
        if is_key_down(KeyCode::A) {
            rot.0 += PI / 180.0;
        }
        if is_key_down(KeyCode::D) {
            rot.0 -= PI / 180.0;
        }
        c += 1;
    }
    debug!("handle_movement_controls handled {} entities", c)
}

pub fn handle_fire_controls(world: &mut World) -> Option<(Position, Rotation, Speed, MaxSpeed, VelocityPower, ObjectType)> {
    if is_key_pressed(KeyCode::Key1) {
        let entities = world.query::<(&Position, &Rotation, &UnderControl)>()
            .iter()
            .map(|(e, (&position, &rotation, &control))| (e, position, rotation, control))
            .collect::<Vec<_>>();
        let mut return_obj = (Position { x: 0.0, y: 0.0 }, Rotation(0.0), Speed(1000.0), MaxSpeed(1000.0), VelocityPower::new(1.0), ObjectType::Bullet);
        for e in entities {
            return_obj.0 = e.1;
            return_obj.1 = e.2;
        }
        debug!("Created bullet with {:?}", return_obj);
        Some(return_obj)
    } else {
        None
    }
}