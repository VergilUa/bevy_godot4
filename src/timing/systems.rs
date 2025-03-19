use std::time::Duration;
use bevy::prelude::{Res, ResMut, Time, Virtual};
use crate::timing::components::CTime;

pub fn set_max_delta(
	mut virtual_time: ResMut<Time<Virtual>>,
){
	virtual_time.set_max_delta(Duration::from_millis(CTime::MAX_DELTA_TIME));
}

pub fn compute_custom_time(
	virtual_time: Res<Time<Virtual>>,
	mut custom_time: ResMut<Time<CTime>>,
	mut time: ResMut<Time>
) {
	let context = custom_time.context_mut();

	let mut next_dt = context.godot_dt;

	// Include timescale after everything
	next_dt = next_dt * virtual_time.relative_speed_f64();

	let next_dt_as_duration = Duration::from_secs_f64(next_dt);

	// Copy as is
	context.is_paused = virtual_time.is_paused();

	custom_time.advance_by(next_dt_as_duration);

	*time = custom_time.as_generic();
}

pub fn override_time(
	custom_time: Res<Time<CTime>>,
	mut time: ResMut<Time>
)
{
	*time = custom_time.as_generic();
}

#[allow(unused)]
pub fn compute_custom_time_keep_dt(
	virtual_time: Res<Time<Virtual>>,
	mut custom_time: ResMut<Time<CTime>>,
	mut time: ResMut<Time>
) {
	let context = custom_time.context_mut();
	
	context.frame_count += 1;

	// Copy as is
	context.is_paused = virtual_time.is_paused();

	custom_time.advance_by(virtual_time.delta());

	*time = custom_time.as_generic();
}

// TODO implement fixed time update