use amethyst::ecs::{Component, DenseVecStorage};
use half_matrix::HalfMatrix;

#[repr(u32)]
#[derive(Debug, Clone, PartialOrd, PartialEq, Component, Serialize, Deserialize)]
pub enum ObjectType {
    Scene, // 0
    StartZone, // 1
    EndZone, // 2
    KillZone, // 3
    Player, // 4
    PlayerFeet, // 5
    Dynamic, // 6
    Ignore, // 7
    SegmentZone(u8), // 8
}

impl Default for ObjectType {
    fn default() -> Self {
        ObjectType::Scene
    }
}

pub fn generate_collision_matrix() -> HalfMatrix {
    let mut m = HalfMatrix::new(9);
    m.enable(4,0);
    m.enable(4,1);
    m.enable(4,2);
    m.enable(4,3);
    m.enable(4,6);
    m.enable(4,8);
    m.enable(5,0);
    m.enable(5,6);
    m
}
