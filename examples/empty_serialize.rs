use pitr_util::pitrmap::PitrMap;

fn main() {
	let map = PitrMap::new();
	let serialized_map = map.to_string().unwrap();

	println!("{}", serialized_map);
}
