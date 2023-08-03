use hecs::World;
use macroquad::math::f32;

use crate::components::{Position, Rotation, Speed};

pub fn system_motion(world: &mut World) {
    for (_id, (pos, speed, rotation)) in &mut world.query_mut::<(&mut Position, &Speed, &Rotation)>().into_iter() {
        if speed != &Speed(0.0) {
            let c: f32 = speed.0 / 60.0;
            let a: f32 = rotation.0.cos() * c;
            let b: f32 = rotation.0.sin() * c;
            pos.x += a;
            pos.y += b;
        }
    }
}
