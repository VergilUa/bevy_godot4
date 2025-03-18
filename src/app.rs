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

lazy_static::lazy_static! {
    #[doc(hidden)]
    pub static ref APP_BUILDER_FN: Mutex<Option<Box<dyn Fn(&mut App) + Send>>> = Mutex::new(None);
}

thread_local! {
    static BEVY_APP: RefCell<Option<App>> = RefCell::new(None);
    static PENDING_FUNCTIONS: RefCell<Vec<Box<dyn FnOnce(&mut App)>>> = RefCell::new(Vec::new());
}

#[derive(GodotClass, Default)]
#[class(base=Node)]
pub struct BevyApp { }

impl BevyApp {
    /// Sets the global Bevy app and executes any enqueued functions.
    pub fn set_bevy_app(app: App) {
        BEVY_APP.with(|app_cell| {
            *app_cell.borrow_mut() = Some(app);
        });

        // Drain the pending functions queue and execute each function.
        BEVY_APP.with(|app_cell| {
            if let Some(ref mut app) = *app_cell.borrow_mut() {
                PENDING_FUNCTIONS.with(|pending| {
                    // Drain the queue; note: draining reverses order, so if order matters,
                    // consider using another approach.
                    for func in pending.borrow_mut().drain(..) {
                        func(app);
                    }
                });
            }
        });
    }

    pub fn with_bevy_app<F, R>(f: F) -> Option<R>
    where
        F: FnOnce(&mut App) -> R + 'static,
    {
        BEVY_APP.with(|app_cell| {
            if let Some(ref mut app) = *app_cell.borrow_mut() {
                // App is available, so execute immediately.
                Some(f(app))
            } else {
                // TODO Godot doesn't seems to have a proper lib loading order.
                // TODO Enqueueing should not be necessary if Bevy App loads faster than actual
                // TODO _init / _ready is called.
                //
                // App not set yet; enqueue the function.
                PENDING_FUNCTIONS.with(|pending| {
                    pending.borrow_mut().push(Box::new(move |app| {
                        // We ignore the result here since we can't return it later.
                        f(app);
                    }));
                });
                None
            }
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
            .add_plugins(bevy::hierarchy::HierarchyPlugin)
            .add_plugins(crate::scene::PackedScenePlugin)
            .init_non_send_resource::<crate::scene_tree::SceneTreeRefImpl>();
        // .add_plugins(GodotSignalsPlugin)
        // .add_plugins(GodotInputEventPlugin);

        #[cfg(feature = "assets")]
        app.add_plugins(crate::assets::GodotAssetsPlugin);

        BevyApp::set_bevy_app(app);
    }

    fn process(&mut self, _delta: f64) {
        if godot::classes::Engine::singleton().is_editor_hint() {
            return;
        }

        BevyApp::with_bevy_app(|app| {
            app.insert_resource(GodotVisualFrame);

            if let Err(e) = catch_unwind(AssertUnwindSafe(|| app.update())) {
                eprintln!("bevy app update panicked");
                resume_unwind(e);
            }

            app.world_mut().remove_resource::<GodotVisualFrame>();
        });
    }

    fn physics_process(&mut self, _delta: f64) {
        if godot::classes::Engine::singleton().is_editor_hint() {
            return;
        }

        BevyApp::with_bevy_app(|app| {
            app.insert_resource(GodotPhysicsFrame);

            if let Err(e) = catch_unwind(AssertUnwindSafe(|| app.update())) {
                eprintln!("bevy app update panicked");
                resume_unwind(e);
            }

            app.world_mut().remove_resource::<GodotPhysicsFrame>();
        });
    }
}
