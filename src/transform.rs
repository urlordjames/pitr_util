use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vec3 {
	pub x: f64,
	pub y: f64,
	pub z: f64
}

impl Vec3 {
	pub fn from(n: f64) -> Self {
		Vec3 {
			x: n,
			y: n,
			z: n
		}
	}
}

auto_ops::impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 {
	Vec3 {
		x: a.x + b.x,
		y: a.y + b.y,
		z: a.z + b.z
	}
});

auto_ops::impl_op_ex!(- |a: &Vec3, b: &Vec3| -> Vec3 {
	Vec3 {
		x: a.x - b.x,
		y: a.y - b.y,
		z: a.z - b.z
	}
});

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rotation {
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub w: f64
}

impl Rotation {
	pub fn zero() -> Self {
		Rotation {
			x: 0.0,
			y: 0.0,
			z: 0.0,
			w: 0.0
		}
	}
}
