use serde::{Serialize, Deserialize};
use crate::transform::{Vec3, Rotation};

#[derive(Serialize, Deserialize, Debug)]
pub enum BlockType {
	#[serde(rename="ultrakill.brush-plastic")]
	Plastic,
	#[serde(rename="ultrakill.brush-metal")]
	Metal,
	#[serde(rename="ultrakill.brush-wood")]
	Wood,
	#[serde(rename="ultrakill.brush-grass")]
	Grass,
	#[serde(rename="ultrakill.brush-glass")]
	Glass
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
	// seems to be ignored
	BlockType: u8
}

impl Block {
	#[allow(non_snake_case)]
	pub fn new(ObjectIdentifier: BlockType) -> Self {
		Block {
			BlockType: match &ObjectIdentifier {
				BlockType::Plastic => 0,
				BlockType::Wood => 1,
				BlockType::Metal => 2,
				// no 3???
				BlockType::Grass => 4,
				BlockType::Glass => 5
			},
			ObjectIdentifier: ObjectIdentifier,
			Position: Vec3::from(0.0),
			Rotation: Rotation::zero(),
			Scale: Vec3::from(0.0),
			Kinematic: true,
			BlockSize: Vec3::from(1.0)
		}
	}
}
