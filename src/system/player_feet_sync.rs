use amethyst_rhusics::rhusics_core::collide3d::BodyPose3;
use amethyst::ecs::{System, ReadStorage, WriteStorage, Join};
use crate::component::{PlayerFeetTag, PlayerTag};
use amethyst_rhusics::rhusics_core::{NextFrame, Pose};
use amethyst::core::cgmath::Point3;

pub struct PlayerFeetSync;

impl<'a> System<'a> for PlayerFeetSync {
    type SystemData = (
        ReadStorage<'a, PlayerFeetTag>,
        ReadStorage<'a, PlayerTag>,
        WriteStorage<'a, BodyPose3<f32>>,
        ReadStorage<'a, NextFrame<BodyPose3<f32>>>,
    );

    fn run(&mut self, (player_feets, players, mut body_poses, next_body_poses): Self::SystemData) {
        // Player in scene
        if let Some(player_position) = (&players, &next_body_poses).join().next().map(|e| e.1.value.position().clone()) {
            // TODO: Replace -0.4 by player half_height
            (&player_feets, &mut body_poses).join().next().expect("No player feet but player is in scene.").1.set_position(Point3::new(player_position.x, player_position.y - 0.4, player_position.z));
        }
    }
}