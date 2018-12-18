use amethyst::utils::scene::BasicScenePrefab;
use amethyst::renderer::PosNormTangTex;
use crate::util::AllEvents;
use amethyst::prelude::*;
use crate::component::ObjectType;

pub type ScenePrefab = BasicScenePrefab<Vec<PosNormTangTex>>;
/*pub type Shape = CollisionShape<Primitive3<f32>, BodyPose3<f32>, Aabb3<f32>, ObjectType>;
pub type DefaultPhysicalEntityParts<'a, T> = PhysicalEntityParts<
    'a,
    Primitive3<f32>,
    T,
    Quaternion<f32>,
    Vector3<f32>,
    Vector3<f32>,
    Matrix3<f32>,
    Aabb3<f32>,
    BodyPose3<f32>,
>;
pub type MyPhysicalEntityParts<'a> = DefaultPhysicalEntityParts<'a, ObjectType>;*/
pub type CustomTrans<'a, 'b> = Trans<GameData<'a, 'b>, AllEvents>;