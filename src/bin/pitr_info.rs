use pitr_util::PitrMap;

fn main() {
	let args = std::env::args();

	if args.len() < 2 {
		panic!("specify one or more arguments");
	}

	args.skip(1).for_each(|filename| {
		println!("---info for map {}---", filename);

		let map = PitrMap::from_file(filename.as_str()).unwrap_or_else(|| {
			panic!("cannot open file {}", filename);
		}).unwrap_or_else(|_| {
			panic!("file {} is not a valid map", filename);
		});

		println!("Blocks: {}", map.Blocks.len());
		println!("Props: {}", map.Props.len());
		println!("Enemies: {}", map.Enemies.len());
	});
}
