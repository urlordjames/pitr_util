use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Vec3 {
	pub x: f64,
	pub y: f64,
	pub z: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rotation {
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub w: f64
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
	pub BlockType: u8
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

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Enemy {
	pub ObjectIdentifier: String,
	pub Position: Vec3,
	pub Rotation: Rotation,
	pub Scale: Vec3
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
