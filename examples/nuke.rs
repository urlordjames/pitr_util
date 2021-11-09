use pitr_util::pitrmap::PitrMap;
use pitr_util::prop::{Prop, PropType};

fn main() {
	let mut map = PitrMap::new();

	for x in 0..50 {
		for z in 0..50 {
			let mut explosive = Prop::new(PropType::ExplosiveBarrel);

			explosive.Position.x = x as f64;
			explosive.Position.y = 30f64;
			explosive.Position.z = 400f64 + z as f64;
			explosive.Kinematic = true;

			map.Props.push(explosive);
		}
	}

	let serialized_map = map.to_string().unwrap();
	println!("{}", serialized_map);
}
