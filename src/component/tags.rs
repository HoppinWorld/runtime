use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Component)]
pub struct PlayerFeetTag;

#[derive(Default, Component, Serialize, Deserialize)]
pub struct PlayerTag;