use std::f32;

#[derive(Debug)]
pub struct Position {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Debug)]
pub struct Rotation(pub(crate) f32);

#[derive(Debug)]
pub enum ObjectType {
    Ship,
    Asteroid,
    Bullet,
}

#[derive(Debug, PartialEq)]
pub struct Speed(pub(crate) f32);

#[derive(Debug)]
pub struct Size(pub(crate) f32);

#[derive(Debug)]
pub struct UnderControl;

#[derive(Debug)]
pub struct MaxSpeed(pub(crate) f32);

#[derive(Debug)]
pub struct VelocityPower(pub(crate) f32);
