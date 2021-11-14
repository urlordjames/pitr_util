use pitr_util::pitrmap::PitrMap;
use pitr_util::block::{Block, BlockType};
use pitr_util::transform::Vec3;

fn graph(x: f32, z: f32) -> f32 {
	x.sin() + z.cos()
}

fn main() {
	let stepsize: f32 = 0.5;
	let mut map = PitrMap::new();

	let mut x = -25.0;
	while x < 25.0 {
		x += stepsize;

		let mut z = -25.0;
		while z < 25.0 {
			z += stepsize;

			let mut point = Block::new(BlockType::Metal);

			point.Position.x = x;
			point.Position.y = graph(x, z);
			point.Position.z = z + 400.0;
			point.BlockSize = Vec3::from(stepsize);

			map.Blocks.push(point);
		}
	}

	let serialized_map = map.to_string().unwrap();
	println!("{}", serialized_map);
}
