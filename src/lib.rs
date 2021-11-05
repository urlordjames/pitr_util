use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Vec3 {
	pub x: f64,
	pub y: f64,
	pub z: f64
}

impl Vec3 {
	pub fn zero() -> Self {
		Vec3 {
			x: 0.0,
			y: 0.0,
			z: 0.0
		}
	}

	pub fn normal() -> Self {
		Vec3 {
			x: 1.0,
			y: 1.0,
			z: 1.0
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rotation {
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub w: f64
}

impl Rotation {
	pub fn zero() -> Self {
		Rotation {
			x: 0.0,
			y: 0.0,
			z: 0.0,
			w: 0.0
		}
	}
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
	pub ObjectIdentifier: String,
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
	pub fn new(ObjectIdentifier: String) -> Self {
		Block {
			ObjectIdentifier,
			Position: Vec3::zero(),
			Rotation: Rotation::zero(),
			Scale: Vec3::zero(),
			Kinematic: true,
			BlockSize: Vec3::normal(),
			BlockType: 0
		}
	}
}

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
			Position: Vec3::zero(),
			Rotation: Rotation::zero(),
			Scale: Vec3::normal(),
			Kinematic: false,
			Type: 0
		}
	}
}

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
			Position: Vec3::zero(),
			Rotation: Rotation::zero(),
			Scale: Vec3::normal()
		}
	}
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PitrMap {
	MapName: String,
	MapIdentifier: String,
	SaveVersion: u8,
	GameVersion: String,
	pub Blocks: Vec<Block>,
	pub Props: Vec<Prop>,
	pub Enemies: Vec<Enemy>
}

impl PitrMap {
	pub fn new() -> Self {
		PitrMap {
			MapName: String::from("uk_construct"),
			MapIdentifier: String::from(""),
			SaveVersion: 1,
			GameVersion: String::from("3.0"),
			Blocks: vec!(),
			Props: vec!(),
			Enemies: vec!()
		}
	}

	pub fn from_string(string: &str) -> Result<Self, serde_json::Error> {
		serde_json::from_str(string)
	}

	pub fn from_file(file_name: &str) -> Option<Result<Self, serde_json::Error>> {
		let file_contents = std::fs::read_to_string(file_name);

		match file_contents {
			Ok(file_contents) => Some(Self::from_string(file_contents.as_str())),
			Err(_) => None
		}
	}

	pub fn to_string(&self) -> Result<String, serde_json::Error> {
		serde_json::to_string(self)
	}
}
