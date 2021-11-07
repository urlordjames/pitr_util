use serde::{Serialize, Deserialize};

mod errors;
use errors::{FromStringError, FromFileError};

#[derive(Serialize, Deserialize, Debug)]
pub struct Vec3 {
	pub x: f64,
	pub y: f64,
	pub z: f64
}

impl Vec3 {
	pub fn from(n: f64) -> Self {
		Vec3 {
			x: n,
			y: n,
			z: n
		}
	}
}

impl std::ops::Add for Vec3 {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Vec3 {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z
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
			Position: Vec3::from(0.0),
			Rotation: Rotation::zero(),
			Scale: Vec3::from(0.0),
			Kinematic: true,
			BlockSize: Vec3::from(1.0),
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
			Position: Vec3::from(0.0),
			Rotation: Rotation::zero(),
			Scale: Vec3::from(1.0),
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
			Position: Vec3::from(0.0),
			Rotation: Rotation::zero(),
			Scale: Vec3::from(1.0)
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

	pub fn from_string(string: &str) -> Result<Self, FromStringError> {
		let map: Result<Self, serde_json::Error> = serde_json::from_str(string);

		match map {
			Ok(parsed_map) => {
				if parsed_map.SaveVersion != 1 {
					return Err(FromStringError::UnsupportedSaveVersion(parsed_map.SaveVersion));
				}

				if parsed_map.GameVersion != "3.0" {
					return Err(FromStringError::UnsupportedGameVersion(parsed_map.GameVersion));
				}

				Ok(parsed_map)
			},
			Err(error) => {
				Err(FromStringError::ParseError(error))
			}
		}
	}

	pub fn from_file(file_name: &str) -> Result<Self, FromFileError> {
		let file_contents = std::fs::read_to_string(file_name);

		match file_contents {
			Ok(file_contents) => {
				let map = Self::from_string(file_contents.as_str());

				match map {
					Ok(parsed_map) => Ok(parsed_map),
					Err(kind) => Err(FromFileError::FromStringError(kind))
				}
			},
			Err(kind) => Err(FromFileError::FileError(kind))
		}
	}

	pub fn to_string(&self) -> Result<String, serde_json::Error> {
		serde_json::to_string(self)
	}
}
