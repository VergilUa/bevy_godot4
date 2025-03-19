pub mod systems;
pub mod components;

use bevy::prelude::*;
use bevy::time::{time_system, TimeSystem};
use crate::timing::components::CTime;
use crate::timing::systems::*;

/// Custom time computing logic
pub struct TimingPlugin;

impl Plugin for TimingPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<Time<CTime>>()
		   .add_systems(Startup, set_max_delta)
		   .add_systems(First,
						compute_custom_time.in_set(TimeSystem)
										   .after(time_system),
		   )
		   .add_systems(PreUpdate, override_time)
		   .add_systems(Update, override_time)
		   .add_systems(PostUpdate, override_time);
	}
}

impl TimingPlugin {
	/// Updated internal `CTime`'s Godot related delta time
	pub fn update_godot_dt(delta: f64, app: &mut App) {
		let Some(mut ctime) = app.world_mut().get_resource_mut::<Time<CTime>>() else {
			return;
		};
		
		let context = ctime.context_mut();
		context.godot_dt = delta;
	}

	/// Updated internal `CTime`'s Godot related delta time
	pub fn update_godot_fixed_dt(fixed_delta: f64, app: &mut App) {
		let Some(mut ctime) = app.world_mut().get_resource_mut::<Time<CTime>>() else {
			return;
		};

		let context = ctime.context_mut();
		context.godot_physics_dt = fixed_delta;
	}
}