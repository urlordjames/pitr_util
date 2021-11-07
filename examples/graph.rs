use pitr_util::{PitrMap, Block, Vec3};

fn graph(x: f64, z: f64) -> f64 {
	x.sin() + z.cos()
}

fn main() {
	let stepsize: f64 = 0.5;
	let mut map = PitrMap::new();

	let mut x = -25f64;
	while x < 25.0 {
		x += stepsize;

		let mut z = -25f64;
		while z < 25.0 {
			z += stepsize;

			let mut point = Block::new(String::from("ultrakill.brush-metal"));

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
