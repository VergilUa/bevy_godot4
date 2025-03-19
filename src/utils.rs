use bevy::{
    ecs::schedule::SystemConfigs,
    prelude::*,
};

/// Bevy Resource that is available when the app is updated through `_process` callback
#[derive(Resource)]
pub struct GodotUpdate;

/// Bevy Resource that is available when the app is updated through `_physics_process` callback
#[derive(Resource)]
pub struct GodotFixedUpdate;

/// Adds `as_godot_fixed_update_system` that schedules a system only for the physics frame
pub trait AsGodotFixedUpdate<Params> {
    #[allow(clippy::wrong_self_convention)]
    fn as_godot_fixed_update_system(self) -> SystemConfigs;
}

impl<Params, T: IntoSystem<(), (), Params>> AsGodotFixedUpdate<Params> for T {
    fn as_godot_fixed_update_system(self) -> SystemConfigs {
        self.run_if(resource_exists::<GodotFixedUpdate>)
    }
}

/// Adds `as_godot_update_system` that schedules a system only for the frame
pub trait AsVisualSystem<Params> {
    #[allow(clippy::wrong_self_convention)]
    fn as_godot_update_system(self) -> SystemConfigs;
}

impl<Params, T: IntoSystem<(), (), Params>> AsVisualSystem<Params> for T {
    fn as_godot_update_system(self) -> SystemConfigs {
        self.run_if(resource_exists::<GodotUpdate>)
    }
}