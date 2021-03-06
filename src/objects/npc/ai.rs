use crate::Tcod;
use crate::environment::{ Game, Map };
use super::{ Object, super::Character };

use rand::Rng;

use serde::{ Serialize, Deserialize };

use tcod::colors::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum Ai {
    Basic,
    Confused {
        previous_ai: Box<Ai>,
        num_turns: i32,
    },
    Fear {
        previous_ai: Box<Ai>,
        num_turns: i32,
    },
}

impl Object {
    // Moves object towards another object.
    fn move_towards(id: usize, target_x: i32, target_y: i32, map: &Map, characters: &mut [Character]) {
        // Vector from this object to the target, and the distance.
        let dx = target_x - characters[id].object.x;
        let dy = target_y - characters[id].object.y;
        let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

        // Normalize to length 1 while keeping direction.
        // Then round, and convert to an integer so movement stays to map grid.
        let dx = (dx as f32 / distance).round() as i32;
        let dy = (dy as f32 / distance).round() as i32;
        Object::move_by(id, dx, dy, map, characters);
    }

    // Calculates distance between object, and another object.
    pub fn distance_to(&self, other: &Object) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }

    // Depending on the current AI status of the object, activates the relevant AI function.
    pub fn ai_take_turn(monster_id: usize, tcod: &Tcod, game: &mut Game, characters: &mut Vec<Character>, player: &mut Object) {
        use Ai::*;
        if let Some(ai) = characters[monster_id].object.ai.take() {
            let new_ai = match ai {
                Basic => Object::ai_basic(monster_id, tcod, game, characters, player),
                Confused{previous_ai, num_turns} => Object::ai_confused(monster_id, tcod, game, characters, previous_ai, num_turns),
                Fear{previous_ai, num_turns} => Object::ai_fear(monster_id, tcod, game, characters, previous_ai, num_turns),
            };
            characters[monster_id].object.ai = Some(new_ai);
        }
    }

    // Because the AI state can change, the different AI types return an AI to insert into the object.
    fn ai_basic(monster_id: usize, tcod: &Tcod, game: &mut Game, characters: &mut [Character], player: &mut Object) -> Ai {
        // A basic monster taking its turn normally.
        // If you can see it, it can see you too.
        let (monster_x, monster_y) = characters[monster_id].object.pos();
        if tcod.fov.is_in_fov(monster_x, monster_y) {
            if characters[monster_id].object.distance_to(player) >= 2.0 {
                // Moves towards player if far away.
                let (player_x, player_y) = player.pos();
                Object::move_towards(monster_id, player_x, player_y, &game.map, characters);
            } else if player.fighter.map_or(false, |f| f.hp > 0) {
                // Close enough to attack.
                characters[monster_id].object.monster_attack(game, player);
            }
        }
        Ai::Basic
    }

    // Returns AI confused, until the confusion wears off, then it returns its previous AI.
    fn ai_confused(
        monster_id: usize,
        _tcod: &Tcod,
        game: &mut Game,
        characters: &mut [Character],
        previous_ai: Box<Ai>,
        num_turns: i32
    ) -> Ai {
        if num_turns >= 0 {
            // Still confused ...
            // Move in a random direction, and decrease the number of turns confused.
            Object::move_by(
                monster_id,
                rand::thread_rng().gen_range(-1, 2),
                rand::thread_rng().gen_range(-1, 2),
                &game.map,
                characters,
            );
            Ai::Confused {
                previous_ai: previous_ai,
                num_turns: num_turns - 1,
            }
        } else {
            // Restore the previous AI, and delete this one.
            game.messages.add(
                format!("The {} is no longer confused!", characters[monster_id].object.name),
                RED,
            );
            *previous_ai
        }
    }

    // Returns AI confused, until the confusion wears off, then it returns its previous AI.
    fn ai_fear(
        monster_id: usize,
        _tcod: &Tcod,
        game: &mut Game,
        characters: &mut [Character],
        previous_ai: Box<Ai>,
        num_turns: i32,
    ) -> Ai {
        if num_turns >= 0 {
            // Still scared ...
            // Stay frozen
            Ai::Fear {
                previous_ai: previous_ai,
                num_turns: num_turns - 1,
            }
        } else {
            // Restore the previous AI, and delete this one.
            game.messages.add(
                format!("The {} is no longer scared!", characters[monster_id].object.name),
                RED,
            );
            *previous_ai
        }
    }

    // Just a simple attack on another object
    fn monster_attack(&self, game: &mut Game, mut other: &mut Object) {
        let mut rng = rand::thread_rng();
        let attack = (self.fighter.map_or(1, |f| f.power)) as f32 + rng.gen_range(-1.0, 1.0);
        let defense = (other.fighter.map_or(1, |f| f.defense)) as f32 + rng.gen_range(-1.0, 1.0);
        let mut level_mod = ((self.level - other.level) / 3) as f32;
        if level_mod <= 0.0 { level_mod = 1.0; }

        let damage = ((attack * level_mod) - defense).round() as i32;
        if damage > 0 {
            // Target takes damage.
            game.messages.add(
                format!(
                    "{} attacks {} dealing {} damage.",
                    self.name, other.name, damage
                ),
                self.color,
            );
            Object::player_damage(damage, game, &mut other);
        } else {
            game.messages.add(
                format!(
                    "{} attacks {} but it has no effect!",
                    self.name, other.name
                ),
                WHITE,
            );
        }
    }


}
