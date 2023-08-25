use hecs::World;
use macroquad::logging::debug;
use macroquad::math::f32;

use crate::components::{MaxSpeed, Position, Rotation, Speed, VelocityPower};

pub fn position_update(world: &mut World) {
    let mut c: usize = 0;
    for (_id, (pos, speed, rotation)) in &mut world.query_mut::<(&mut Position, &Speed, &Rotation)>().into_iter() {
        if speed != &Speed(0.0) {
            let c: f32 = speed.0 / 60.0;
            let a: f32 = rotation.0.cos() * c;
            let b: f32 = rotation.0.sin() * c;
            pos.x += a;
            pos.y += b;
        }
        c += 1;
    }
    debug!("position_update handled {} entities", c)
}

pub fn speed_update(world: &mut World) {
    let mut c: usize = 0;
    for (_id, (speed, max_speed, velocity_power)) in &mut world.query_mut::<(&mut Speed, &MaxSpeed, &VelocityPower)>().into_iter() {
        speed.0 = max_speed.0 * velocity_power.get_mpl();
        c += 1
    }
    debug!("speed_update handled {} entities", c)
}