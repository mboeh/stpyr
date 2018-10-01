use super::{curses::Glyph, grid::*, pos::*, vault::*};
use failure::Error;
use specs::{prelude::*, storage::BTreeStorage};

#[derive(Debug, Clone)]
pub struct Tile {
    pub glyph:  Glyph,
    pub solid:  bool,
    pub opaque: bool,
}

impl Default for Tile {
    fn default() -> Tile {
        Tile {
            glyph:  Glyph('.'),
            solid:  false,
            opaque: false,
        }
    }
}

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct TileMap {
    pub tiles: Grid<Tile>,
}

impl TileMap {
    pub fn new(width: u32, height: u32) -> TileMap {
        TileMap {
            tiles: Grid::new(width, height, Tile::default()),
        }
    }

    pub fn at(&self, pos: Pos) -> &Tile { self.tiles.at(pos) }

    pub fn contains(&self, pos: Pos) -> bool { self.tiles.contains(pos) }

    pub fn place_vault(&mut self, vault: &Vault) -> Result<(), BlitError> {
        self.tiles.blit(5, 5, &vault.tiles)
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Location {
    pub map: Entity,
    pub pos: Pos,
}

impl HasPos for Location {
    fn pos(&self) -> &Pos { &self.pos }

    fn set_pos(&mut self, pos: Pos) { self.pos = pos; }
}
