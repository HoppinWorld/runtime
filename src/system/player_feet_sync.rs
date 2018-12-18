
use amethyst::ecs::{System, ReadStorage, WriteStorage, Join};
use crate::component::{PlayerFeetTag, PlayerTag};
use amethyst::core::Transform;
use amethyst::core::nalgebra::Vector3;

pub struct PlayerFeetSync;

impl<'a> System<'a> for PlayerFeetSync {
    type SystemData = (
        ReadStorage<'a, PlayerFeetTag>,
        ReadStorage<'a, PlayerTag>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (player_feets, players, mut transforms): Self::SystemData) {
        // Player in scene
        if let Some(player_position) = (&players, &transforms).join().next().map(|e| e.1.translation().clone()) {
            // TODO: Replace -0.4 by player half_height
            *(&player_feets, &mut transforms).join().next().expect("No player feet but player is in scene.").1.translation_mut() = Vector3::new(player_position.x, player_position.y - 0.4, player_position.z);
        }
    }
}