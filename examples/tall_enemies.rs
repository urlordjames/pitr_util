use pitr_util::pitrmap::PitrMap;

fn main() {
	let mut map = PitrMap::from_file("test.pitr").expect("file test.pitr does not exist").expect("invalid map file");

	for enemy in map.Enemies.iter_mut() {
		enemy.Scale.y = 1f64;
	}

	let reserialized_map = map.to_string().expect("cannot reserialize map");
	println!("{}", reserialized_map);
}
