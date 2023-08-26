use std::f32;

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Rotation(pub(crate) f32);

#[derive(Debug, Copy, Clone)]
pub enum ObjectType {
    Ship,
    Asteroid,
    Bullet,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Speed(pub(crate) f32);

#[derive(Debug)]
pub struct Size(pub(crate) f32);

#[derive(Debug, Copy, Clone)]
pub struct UnderControl;

#[derive(Debug, Copy, Clone)]
pub struct MaxSpeed(pub(crate) f32);

#[derive(Debug, Copy, Clone)]
pub struct VelocityPower(f32);

impl VelocityPower {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn get_mpl(&self) -> f32 {
        self.0
    }

    pub fn set_mpl(&mut self, new_value: f32) {
        self.0 = new_value
    }

    pub fn speed_up(&mut self) {
        if self.get_mpl() < 1.0 {
            self.set_mpl(self.get_mpl() + 0.1)
        }
    }

    pub fn speed_down(&mut self) {
        if self.get_mpl() > -0.2 {
            self.set_mpl(self.get_mpl() - 0.1)
        }
    }
}
