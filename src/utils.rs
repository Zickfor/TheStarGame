use hecs::{Entity, World};
use crate::components::UnderControl;

pub(crate) fn get_ship(world: &World) -> Entity {
    world.query::<(&UnderControl)>()
        .iter()
        .map(|(e, (u))| (e, u))
        .collect::<Vec<_>>()
        .get(0)
        .unwrap()
        .0
}