use serde::{Serialize, Deserialize};
use crate::transform::{Vec3, Rotation};

#[derive(Serialize, Deserialize, Debug)]
pub enum EnemyType {
	#[serde(rename="ultrakill.filth")]
	Filth,
	#[serde(rename="ultrakill.stray")]
	Stray,
	#[serde(rename="ultrakill.schism")]
	Schism,
	#[serde(rename="ultrakill.soldier")]
	Soldier,
	#[serde(rename="ultrakill.stalker")]
	Stalker,
	#[serde(rename="ultrakill.insurrectionist")]
	Insurrectionist,
	#[serde(rename="ultrakill.swordsmachine")]
	Swordsmachine,
	#[serde(rename="ultrakill.drone")]
	Drone,
	#[serde(rename="ultrakill.streetcleaner")]
	Streetcleaner,
	#[serde(rename="ultrakill.mindflayer")]
	Mindflayer,
	#[serde(rename="ultrakill.malicious-face")]
	MaliciousFace,
	#[serde(rename="ultrakill.cerberus")]
	Cerberus,
	#[serde(rename="ultrakill.mass")]
	HideousMass,
	#[serde(rename="ultrakill.virtue")]
	Virtue
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Enemy {
	pub ObjectIdentifier: EnemyType,
	pub Position: Vec3,
	pub Rotation: Rotation,
	pub Scale: Vec3
}

impl Enemy {
	#[allow(non_snake_case)]
	pub fn new(ObjectIdentifier: EnemyType) -> Self {
		Enemy {
			Scale: match &ObjectIdentifier {
				EnemyType::Filth => Vec3::from(0.225),
				EnemyType::Stray => Vec3::from(0.35),
				EnemyType::Schism => Vec3::from(0.3),
				EnemyType::Soldier => Vec3::from(2.5),
				EnemyType::Stalker => Vec3::from(1.0),
				EnemyType::Insurrectionist => Vec3::from(1.0),
				EnemyType::Swordsmachine => Vec3::from(1.0),
				EnemyType::Drone => Vec3::from(2.0),
				EnemyType::Streetcleaner => Vec3::from(1.0),
				EnemyType::Mindflayer => Vec3::from(1.0),
				EnemyType::MaliciousFace => Vec3::from(1.0),
				EnemyType::Cerberus => Vec3::from(4.0),
				EnemyType::HideousMass => Vec3::from(1.0),
				EnemyType::Virtue => Vec3::from(1.0)
			},
			ObjectIdentifier,
			Position: Vec3::from(0.0),
			Rotation: Rotation::zero(),
		}
	}
}
