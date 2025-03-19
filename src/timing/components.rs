use bevy::prelude::*;

/// Custom time data for the game
/// (taken from the Godot via interop)
#[derive(Default, Resource, Reflect)]
pub struct CTime {
	pub frame_count: u64,

	// Simplified data access (taken from Time<Virtual> directly)
	pub is_paused: bool,

	pub godot_dt: f64,
	pub godot_physics_dt: f64
}

impl CTime {
	/// Maximum delta time set for the application (in milliseconds)
	pub const MAX_DELTA_TIME: u64 = 250;
}