use amethyst::ecs::{Component, DenseVecStorage};

pub struct Velocity(f32, f32, f32);
impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

pub struct Target(pub f32, pub f32, pub f32);
impl Component for Target {
    type Storage = DenseVecStorage<Self>;
}

pub enum UnitType {
    Battler,
}

pub struct Unit {
    pub unit_type: UnitType, 
}

impl Component for Unit {
    type Storage = DenseVecStorage<Self>;
}

pub struct AxisAlignedHitbox {
    vertex1: (f32, f32, f32),
    vertex2: (f32, f32, f32),
}
impl Component for AxisAlignedHitbox {
    type Storage = DenseVecStorage<Self>;
}

pub struct Speed(pub f32);

impl Component for Speed {
    type Storage = DenseVecStorage<Self>;
}
