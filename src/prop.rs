use serde::{Serialize, Deserialize};
use crate::transform::{Vec3, Rotation};

#[derive(Serialize, Deserialize, Debug)]
pub enum PropType {
	#[serde(rename="ultrakill.barrel")]
	Barrel,
	#[serde(rename="ultrakill.explosive-barrel")]
	ExplosiveBarrel,
	#[serde(rename="ultrakill.crate")]
	Crate,
	#[serde(rename="ultrakill.barrier")]
	Barrier,
	#[serde(rename="ultrakill.maurice")]
	Maurice,
	#[serde(rename="ultrakill.melon")]
	Melon,
	#[serde(rename="ultrakill.tree")]
	Tree,
	#[serde(rename="ultrakill.jump-pad")]
	JumpPad,
	#[serde(rename="ultrakill.grapple-point")]
	GrapplePoint
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Prop {
	pub ObjectIdentifier: PropType,
	pub Position: Vec3,
	pub Rotation: Rotation,
	pub Scale: Vec3,
	pub Kinematic: bool,
	// always 0
	Type: u8
}

impl Prop {
	#[allow(non_snake_case)]
	pub fn new(ObjectIdentifier: PropType) -> Self {
		Prop {
			Scale: match &ObjectIdentifier {
				PropType::Maurice => Vec3::from(0.15),
				_ => Vec3::from(1.0)
			},
			ObjectIdentifier,
			Position: Vec3::from(0.0),
			Rotation: Rotation::zero(),
			Kinematic: false,
			Type: 0
		}
	}
}
