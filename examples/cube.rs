use pitr_util::pitrmap::PitrMap;
use pitr_util::block::{Block, BlockType};
use pitr_util::transform::Vec3;

fn main() {
	let mut map = PitrMap::new();

	let mut block = Block::new(BlockType::Metal);
	block.Position = Vec3 {
		x: -60.0,
		y: -10.0,
		z: 370.0
	};
	map.Blocks.push(block);

	let serialized_map = map.to_string().unwrap();
	println!("{}", serialized_map);
}
