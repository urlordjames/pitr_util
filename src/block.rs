use serde::{Serialize, Deserialize};
use crate::transform::{Vec3, Rotation};

#[derive(Serialize, Deserialize, Debug)]
pub enum BlockType {
	#[serde(rename="ultrakill.brush-metal")]
	Metal
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
	pub ObjectIdentifier: BlockType,
	pub Position: Vec3,
	pub Rotation: Rotation,
	pub Scale: Vec3,
	pub Kinematic: bool,
	pub BlockSize: Vec3,
	// seems to be ignored, use ObjectIdentifier instead
	BlockType: u8
}

impl Block {
	#[allow(non_snake_case)]
	pub fn new(ObjectIdentifier: BlockType) -> Self {
		Block {
			ObjectIdentifier,
			Position: Vec3::from(0.0),
			Rotation: Rotation::zero(),
			Scale: Vec3::from(0.0),
			Kinematic: true,
			BlockSize: Vec3::from(1.0),
			BlockType: 0
		}
	}
}
