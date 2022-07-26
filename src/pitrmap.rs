use serde::{Serialize, Deserialize};
use crate::block::Block;
use crate::prop::Prop;
use crate::enemy::Enemy;
use crate::errors::{FromStringError, FromFileError};

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

	pub fn map_name(&self) -> &str {
		&self.MapName
	}

	pub fn to_string(&self) -> Result<String, serde_json::Error> {
		serde_json::to_string(self)
	}
}
