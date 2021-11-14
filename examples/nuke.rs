use pitr_util::pitrmap::PitrMap;
use pitr_util::prop::{Prop, PropType};

fn main() {
	let mut map = PitrMap::new();

	for x in 0..50 {
		for z in 400..450 {
			let mut explosive = Prop::new(PropType::ExplosiveBarrel);

			explosive.Position.x = x as f32;
			explosive.Position.y = 30.0;
			explosive.Position.z = z as f32;
			explosive.Kinematic = true;

			map.Props.push(explosive);
		}
	}

	let serialized_map = map.to_string().unwrap();
	println!("{}", serialized_map);
}
