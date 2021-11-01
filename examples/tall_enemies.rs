use pitr_util::pitrmap::PitrMap;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let filename = args.get(1).expect("specify a file argument");

	let mut map = PitrMap::from_file(filename).expect("no such file").expect("invalid map file");

	for enemy in map.Enemies.iter_mut() {
		enemy.Scale.y += 1f64;
	}

	let reserialized_map = map.to_string().expect("cannot reserialize map");
	println!("{}", reserialized_map);
}
