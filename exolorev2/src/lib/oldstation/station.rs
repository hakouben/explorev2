pub struct Station {
    position: (f32, f32),
    energie: f32,
    minerais: f32,
    donnees_scientifiques: Vec<Donnee>,
    robots: Vec<Robot>,
}

pub struct Donnee {
    id: u32,
    valeur: String,
    id_robot: u32,
}
pub fn spawn_station(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: RES<AssetServer>,
) {
    let window: &Window = window_query.single().unwrap();
    commands.spawn_bundle(SpriteBundle {
        material: asset_server.load("station.png"),
        transform: Transform::from_translation(Vec3::new(
            window.width() / 2.0,
            window.height() / 2.0,
            0.0,
        )),
        ..Default::default()
    });
}
