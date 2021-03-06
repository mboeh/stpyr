use failure::Error;
use super::resources::*;

pub mod terrain;
pub use self::terrain::*;

pub mod vault;
pub use self::vault::*;

pub trait Definition {
    fn mint(self, builder: specs::EntityBuilder<'_>) -> specs::EntityBuilder<'_>;
}

pub trait Codex<D: Definition>: LoadableResource {
    type ID;

    fn load<L: ResourceDataLoader>(loader: &L) -> Option<Self>;
    fn get(&self, id: Self::ID) -> Option<D>;
}

#[derive(Deserialize, Clone)]
pub struct ActorDef {
    pub id:          String,
    pub glyph:       char,
    pub name:        String,
    pub description: String,
    pub speed:       f32,
}

impl Definition for ActorDef {
    fn mint(self, builder: specs::EntityBuilder<'_>) -> specs::EntityBuilder<'_> {
        use specs::Builder;

        builder
            .with(super::appearance::Appearance {
                glyph:       super::appearance::Glyph::new(self.glyph),
                name:        self.name,
                description: self.description,
            })
            .with(super::energy::Energy::new(self.speed))
            .with(super::action::Turn::default())
            .with(super::fov::FovMap::default())
            .with(super::movement::MovementMap::default())
    }
}

#[derive(Deserialize)]
pub struct Bestiary {
    bestiary: Vec<ActorDef>,
}

pub struct BestiaryLoader;

impl ResourceLoader<Bestiary> for BestiaryLoader {
    fn load(&self, data: String) -> Result<Bestiary, Error> {
        Ok(toml::from_str(&data)?)
    }
}

impl LoadableResource for Bestiary {
    type Loader = BestiaryLoader;
}

impl Codex<ActorDef> for Bestiary {
    type ID = String;

    fn load<L: ResourceDataLoader>(loader: &L) -> Option<Self> {
        loader.load("bestiary.toml", BestiaryLoader).ok()
    }

    fn get(&self, id: Self::ID) -> Option<ActorDef> {
        let it = self.bestiary.iter().find(|e| e.id == id)?;
        Some(it.clone())
    }
}
