#[macro_use]
extern crate log;

pub mod document;
pub mod proto;

pub mod executor;
pub mod imaginate_input;

/// (a hack)
/// This type makes glam::DVec2 compatible with ts-rs
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, ts_rs::TS)]
pub struct DVec2 {
	pub x: f64,
	pub y: f64,
}

impl From<glam::DVec2> for DVec2 {
	fn from(glam::DVec2 { x, y }: glam::DVec2) -> Self {
		Self { x, y }
	}
}

impl Into<glam::DVec2> for DVec2 {
	fn into(self) -> glam::DVec2 {
		let Self { x, y } = self;
		glam::DVec2 { x, y }
	}
}
