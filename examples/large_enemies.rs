use pitr_util::pitrmap::PitrMap;
use pitr_util::transform::Vec3;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let filename = args.get(1).expect("specify a file argument");

	let mut map = PitrMap::from_file(filename).expect("a valid file needs to be present");

	for enemy in map.Enemies.iter_mut() {
		enemy.Scale = &enemy.Scale * Vec3::from(1.2);
	}

	println!("{}", map.to_string().unwrap());
}
