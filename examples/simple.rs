use bevy::prelude::*;
use bevy_spine::{
    SkeletonController, SkeletonData, Spine, SpineBundle, SpinePlugin, SpineReadyEvent, SpineSet,
};
#[cfg(feature = "egui_debugger")]
use {
    bevy_egui::{EguiContexts, EguiPlugin},
    bevy_spine::debugger::egui::egui_spine_debugger,
};

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, SpinePlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, on_spawn.in_set(SpineSet::OnReady));
    #[cfg(feature = "egui_debugger")]
    app.add_plugins(EguiPlugin)
        .add_systems(Update, spine_debugger);
    app.run();
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut skeletons: ResMut<Assets<SkeletonData>>,
) {
    commands.spawn(Camera2dBundle::default());

    let skeleton = {
        #[cfg(feature = "spine38")]
        {
            SkeletonData::new_from_json(
                asset_server.load("spineboy-3.8/export/spineboy-pro.json"),
                asset_server.load("spineboy-3.8/export/spineboy-pma.atlas"),
            )
        }
        #[cfg(not(feature = "spine38"))]
        {
            SkeletonData::new_from_json(
                asset_server.load("spineboy/export/spineboy-pro.json"),
                asset_server.load("spineboy/export/spineboy-pma.atlas"),
            )
        }
    };
    let skeleton_handle = skeletons.add(skeleton);

    commands.spawn(SpineBundle {
        skeleton: skeleton_handle.clone(),
        transform: Transform::from_xyz(0., -200., 0.),
        ..Default::default()
    });
}

fn on_spawn(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut spine_query: Query<&mut Spine>,
) {
    for event in spine_ready_event.read() {
        if let Ok(mut spine) = spine_query.get_mut(event.entity) {
            let Spine(SkeletonController {
                skeleton,
                animation_state,
                ..
            }) = spine.as_mut();
            skeleton.set_scale(Vec2::splat(0.5));
            let _ = animation_state.set_animation_by_name(0, "portal", true);
        }
    }
}

#[cfg(feature = "egui_debugger")]
fn spine_debugger(mut egui_context: EguiContexts, mut spine_query: Query<&mut Spine>) {
    for mut spine in spine_query.iter_mut() {
        let Spine(controller) = spine.as_mut();
        let SkeletonController {
            skeleton,
            animation_state,
            ..
        } = controller;
        egui_spine_debugger(egui_context.ctx_mut(), "Spine", skeleton, animation_state);
    }
}
