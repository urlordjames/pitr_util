use pitr_util::{PitrMap, Block, Vec3};

fn main() {
	let mut map = PitrMap::new();

	let mut block = Block::new(String::from("ultrakill.brush-metal"));
	block.Position = Vec3 {
		x: -60.0,
		y: -10.0,
		z: 370.0
	};
	block.BlockSize = Vec3::normal();
	map.Blocks.push(block);

	let serialized_map = map.to_string().unwrap();
	println!("{}", serialized_map);
}
