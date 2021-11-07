use pitr_util::{PitrMap, Block, Vec3};

fn graph(x: f64, z: f64) -> f64 {
	x.sin() + z.cos()
}

fn main() {
	let stepsize: f64 = 0.5;
	let mut map = PitrMap::new();

	let mut x = 0f64;
	while x < 50.0 {
		x += stepsize;

		let mut z = 400f64;
		while z < 450.0 {
			z += stepsize;

			let mut point = Block::new(String::from("ultrakill.brush-metal"));

			point.Position.x = x;
			point.Position.y = graph(x, z);
			point.Position.z = z;
			point.BlockSize = Vec3::from(stepsize);

			map.Blocks.push(point);
		}
	}

	let serialized_map = map.to_string().unwrap();
	println!("{}", serialized_map);
}
