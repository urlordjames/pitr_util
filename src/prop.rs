use serde::{Serialize, Deserialize};
use crate::transform::{Vec3, Rotation};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Prop {
	pub ObjectIdentifier: String,
	pub Position: Vec3,
	pub Rotation: Rotation,
	pub Scale: Vec3,
	pub Kinematic: bool,
	pub Type: u8
}

impl Prop {
	#[allow(non_snake_case)]
	pub fn new(ObjectIdentifier: String) -> Self {
		Prop {
			ObjectIdentifier,
			Position: Vec3::from(0.0),
			Rotation: Rotation::zero(),
			Scale: Vec3::from(1.0),
			Kinematic: false,
			Type: 0
		}
	}
}
