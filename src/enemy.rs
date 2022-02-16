use serde::{Serialize, Deserialize};
use crate::transform::{Vec3, Rotation};

#[derive(Serialize, Deserialize, Debug)]
pub enum EnemyType {
	#[serde(rename="ultrakill.flith")]
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
	#[serde(rename="ultrakill.v2")]
	V2First,
	#[serde(rename="ultrakill.mindflayer")]
	Mindflayer,
	#[serde(rename="ultrakill.v2_2")]
	V2Second,
	#[serde(rename="ultrakill.malicious-face")]
	MaliciousFace,
	#[serde(rename="ultrakill.cerberus")]
	Cerberus,
	#[serde(rename="ultrakill.mass")]
	HideousMass,
	#[serde(rename="ultrakill.gabriel")]
	Gabriel,
	#[serde(rename="ultrakill.virtue")]
	Virtue,
	#[serde(rename="ultrakill.cancerous-rodent")]
	CancerousRodent,
	#[serde(rename="ultrakill.very-cancerous-rodent")]
	VeryCancerousRodent,
	#[serde(rename="ultrakill.mandalore")]
	Mandalore,
	#[serde(rename="ultrakill.flesh-prison")]
	FleshPrison,
	#[serde(rename="ultrakill.minos-prime")]
	MinosPrime
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
			Scale: Enemy::type_scale(&ObjectIdentifier),
			ObjectIdentifier,
			Position: Vec3::from(0.0),
			Rotation: Rotation::zero(),
		}
	}

	pub fn set_type(&mut self, enemy_type: EnemyType) {
		self.Scale = Enemy::type_scale(&enemy_type);
		self.ObjectIdentifier = enemy_type;
	}

	fn type_scale(enemy_type: &EnemyType) -> Vec3 {
		match enemy_type {
			EnemyType::Filth => Vec3::from(0.225),
			EnemyType::Stray => Vec3::from(0.35),
			EnemyType::Schism => Vec3::from(0.3),
			EnemyType::Soldier => Vec3::from(2.5),
			EnemyType::Stalker => Vec3::from(1.0),
			EnemyType::Insurrectionist => Vec3::from(1.0),
			EnemyType::Swordsmachine => Vec3::from(1.0),
			EnemyType::Drone => Vec3::from(2.0),
			EnemyType::Streetcleaner => Vec3::from(1.0),
			EnemyType::V2First => Vec3::from(1.0),
			EnemyType::Mindflayer => Vec3::from(1.0),
			EnemyType::V2Second => Vec3::from(1.0),
			EnemyType::MaliciousFace => Vec3::from(1.0),
			EnemyType::Cerberus => Vec3::from(4.0),
			EnemyType::HideousMass => Vec3::from(1.0),
			EnemyType::Gabriel => Vec3::from(1.0),
			EnemyType::Virtue => Vec3::from(1.0),
			EnemyType::CancerousRodent => Vec3::from(0.2),
			EnemyType::VeryCancerousRodent => Vec3::from(3.0),
			EnemyType::Mandalore => Vec3::from(1.0),
			EnemyType::FleshPrison => Vec3::from(4.0),
			EnemyType::MinosPrime => Vec3::from(1.0)
		}
	}
}
