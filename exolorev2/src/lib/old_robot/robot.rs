pub struct Robot {
    id: u32,
    position: (f32, f32),
    energie: f32,
    minerais: f32,
    points_interet_scientifiques: Vec<(f32, f32)>,
    modules: Vec<Module>,
}

pub enum Module {
    AnalyseChimique,
    Forage,
    ImagerieHauteResolution,
}
