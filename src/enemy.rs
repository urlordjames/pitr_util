use serde::{Serialize, Deserialize};
use crate::transform::{Vec3, Rotation};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Enemy {
	pub ObjectIdentifier: String,
	pub Position: Vec3,
	pub Rotation: Rotation,
	pub Scale: Vec3
}

impl Enemy {
	#[allow(non_snake_case)]
	pub fn new(ObjectIdentifier: String) -> Self {
		Enemy {
			ObjectIdentifier,
			Position: Vec3::from(0.0),
			Rotation: Rotation::zero(),
			Scale: Vec3::from(1.0)
		}
	}
}
