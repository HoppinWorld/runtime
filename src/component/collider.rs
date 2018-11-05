use amethyst::ecs::{Component, DenseVecStorage};
use amethyst_rhusics::rhusics_core::Collider;

#[repr(u8)]
#[derive(Debug, Clone, PartialOrd, PartialEq, Component, Serialize, Deserialize)]
pub enum ObjectType {
    Scene,
    StartZone,
    EndZone,
    KillZone,
    Player,
    PlayerFeet,
    Dynamic,
    Ignore,
    SegmentZone(u8),
}

impl Default for ObjectType {
    fn default() -> Self {
        ObjectType::Scene
    }
}

fn contact_dual(s: &ObjectType, o: &ObjectType, w1: &ObjectType, w2: &ObjectType) -> bool {
    // Special match to ignore the segment zone id
    if let &ObjectType::SegmentZone(_) = s {
        return match (w1, w2) {
            (&ObjectType::SegmentZone(_), other) => o == other,
            (other, &ObjectType::SegmentZone(_)) => o == other,
            _ => false
        };
    }

    if let &ObjectType::SegmentZone(_) = o {
        return match (w1, w2) {
            (&ObjectType::SegmentZone(_), other) => s == other,
            (other, &ObjectType::SegmentZone(_)) => s == other,
            _ => false
        };
    }

    (s == w1 && o == w2) || (s == w2 && o == w1)
}

impl Collider for ObjectType {
    fn should_generate_contacts(&self, other: &ObjectType) -> bool {
        contact_dual(self, other, &ObjectType::Player, &ObjectType::Scene) ||
        contact_dual(self, other, &ObjectType::PlayerFeet, &ObjectType::Scene) ||
        contact_dual(self, other, &ObjectType::Player, &ObjectType::KillZone) ||
        contact_dual(self, other, &ObjectType::Player, &ObjectType::StartZone) ||
        contact_dual(self, other, &ObjectType::Player, &ObjectType::EndZone) ||
        contact_dual(self, other, &ObjectType::Player, &ObjectType::Dynamic) ||
        contact_dual(self, other, &ObjectType::Player, &ObjectType::SegmentZone(69))
        //*self == ObjectType::Player || *other == ObjectType::Player || *self == ObjectType::Dynamic || *other == ObjectType::Dynamic
    }
}