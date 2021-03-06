use crate::environment::{ Game, Map };

pub mod player;
pub mod npc;
use npc::*;
use npc::ai::*;

pub mod items;
use items::*;

use serde::{ Serialize, Deserialize };

use tcod::colors::*;
use tcod::console::*;

// Object struct definition.
#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    pub x: i32,
    pub y: i32,
    pub char: char,
    pub color: Color,
    pub name: String,
    pub blocks: bool,
    pub alive: bool,
    pub corpse_type: String,
    pub fighter: Option<Fighter>,
    pub ai: Option<Ai>,
    pub item: Option<Item>,
    pub level: i32,
    pub always_visible: bool,
}

// Character definition
#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub object: Object,
    pub inventory: Option<Vec<Object>>
}

// Item definition
impl Object {
    // Places object on the screen
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    // Checks to see if an object is meant to block other objects.
    pub fn is_blocked(x: i32, y: i32, map: &Map, characters: &[Character]) -> bool {
        // First test the map tile
        if map[x as usize][y as usize].blocked {
            return true;
        }
        // Checks for any blocking objects
        characters.iter().any(|character| character.object.blocks && character.object.pos() == (x, y))
    }

    // Returns the x/y coordinates of the object.
    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    // Sets the x/y coordinates of the object.
    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    // Moves unit in a direction if the tile isn't blocked
    pub fn move_by(id: usize, dx: i32, dy: i32, map: &Map, objects: &mut [Character]) {
        let (x, y) = objects[id].object.pos();
        if !Object::is_blocked(x + dx, y + dy, &map, objects) {
            objects[id].object.set_pos(x + dx, y + dy);
        }
    }

    // Function to allow fighter-enabled objects to take damage
    fn take_damage(&mut self, damage: i32, game: &mut Game) -> Option<i32> {
        // Apply damage if possible.
        if let Some(fighter) = self.fighter.as_mut() {
            if damage > 0 {
                fighter.hp -= damage;
            }
        }

        // Check for death, and possibly call death function.
        if let Some(fighter) = self.fighter {
            if fighter.hp <= 0 {
                self.alive = false;
                fighter.on_death.callback(self, game);
                return Some(fighter.exp);
            }
        }
        None
    }

    // Find distance between self, and another target.
    pub fn distance(&self, x: i32, y: i32) -> f32 {
        (((x - self.x).pow(2) + (y - self.y).pow(2)) as f32).sqrt()
    }

    /// heal by the given amount, without going over the maximum
    pub fn heal(&mut self, amount: i32) {
        if let Some(ref mut fighter) = self.fighter {
            fighter.hp += amount;
            if fighter.hp > fighter.max_hp {
                fighter.hp = fighter.max_hp;
            }
        }
    }
}
