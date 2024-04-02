use bevy::pbr::Lightmap;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .register_type::<Lightmap>()
        .add_systems(Startup, load_gltf)
        .add_systems(Update, add_lightmap)
        .run();
}

fn add_lightmap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mesh_query: Query<(Entity, &Name), Added<Handle<Mesh>>>,
) {
    for (ent, name) in &mesh_query {
        if name.contains("Plane") {
            commands.entity(ent).insert(Lightmap {
                image: asset_server.load("Textures/Plane_baked.png"),
                ..default()
            });
        }

        if name.contains("Cube") {
            commands.entity(ent).insert(Lightmap {
                image: asset_server.load("Textures/Cube_baked.png"),
                ..default()
            });
        }
    }
}

fn load_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("simple.gltf#Scene0"),
        ..default()
    });
}
