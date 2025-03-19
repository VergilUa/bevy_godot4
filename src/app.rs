use bevy::app::App;
use godot::{
    classes::{INode, Node},
    obj::Base,
    prelude::{godot_api, GodotClass},
};

use crate::prelude::*;
use std::{
    panic::{catch_unwind, resume_unwind, AssertUnwindSafe},
    cell::RefCell,
    sync::Mutex,
};
use crate::timing::TimingPlugin;

lazy_static::lazy_static! {
    #[doc(hidden)]
    pub static ref APP_BUILDER_FN: Mutex<Option<Box<dyn Fn(&mut App) + Send>>> = Mutex::new(None);
}

thread_local! {
    static BEVY_APP: RefCell<Option<App>> = RefCell::new(None);
}

#[derive(GodotClass, Default)]
#[class(base=Node)]
pub struct BevyApp { }

impl BevyApp {
    pub fn set_bevy_app(app: App) {
        BEVY_APP.with(|app_cell| {
            *app_cell.borrow_mut() = Some(app);
        });
    }

    pub fn with_bevy_app<F, R>(f: F) -> Option<R>
    where
        F: FnOnce(&mut App) -> R,
    {
        BEVY_APP.with(|app_cell| {
            let mut borrow = app_cell.borrow_mut();
            borrow.as_mut().map(|app| f(app))
        })
    }
}

#[godot_api]
impl INode for BevyApp {
    fn init(_base: Base<Node>) -> Self {
        Default::default()
    }

    fn ready(&mut self) {
        if godot::classes::Engine::singleton().is_editor_hint() {
            return;
        }

        let mut app = App::new();
        (APP_BUILDER_FN.lock().unwrap().as_mut().unwrap())(&mut app);
        app.add_plugins(bevy::core::TaskPoolPlugin::default())
            .add_plugins(bevy::log::LogPlugin::default())
            .add_plugins(bevy::core::TypeRegistrationPlugin)
            .add_plugins(bevy::core::FrameCountPlugin)
            .add_plugins(bevy::diagnostic::DiagnosticsPlugin)
            .add_plugins(bevy::time::TimePlugin)
            .add_plugins(TimingPlugin)
            .add_plugins(bevy::hierarchy::HierarchyPlugin)
            .add_plugins(crate::scene::PackedScenePlugin)
            .init_non_send_resource::<crate::scene_tree::SceneTreeRefImpl>();
        // .add_plugins(GodotSignalsPlugin)
        // .add_plugins(GodotInputEventPlugin);

        #[cfg(feature = "assets")]
        app.add_plugins(crate::assets::GodotAssetsPlugin);

        BevyApp::set_bevy_app(app);
    }

    fn process(&mut self, delta_time: f64) {
        if godot::classes::Engine::singleton().is_editor_hint() {
            return;
        }

        BevyApp::with_bevy_app(|app| {
            TimingPlugin::update_godot_dt(delta_time, app);
            
            app.insert_resource(GodotUpdate);

            if let Err(e) = catch_unwind(AssertUnwindSafe(|| app.update())) {
                eprintln!("bevy app update panicked");
                resume_unwind(e);
            }

            app.world_mut().remove_resource::<GodotUpdate>();
        });
    }

    fn physics_process(&mut self, fixed_delta: f64) {
        if godot::classes::Engine::singleton().is_editor_hint() {
            return;
        }

        BevyApp::with_bevy_app(|app| {
            TimingPlugin::update_godot_fixed_dt(fixed_delta, app);
            
            app.insert_resource(GodotFixedUpdate);

            if let Err(e) = catch_unwind(AssertUnwindSafe(|| app.update())) {
                eprintln!("bevy app update panicked");
                resume_unwind(e);
            }

            app.world_mut().remove_resource::<GodotFixedUpdate>();
        });
    }
}
