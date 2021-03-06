use crate::*;
use crate::environment::Game;
use crate::Tcod;
use crate::objects::*;
use crate::graphics::gui::menu::inventory_menu;

use serde::{ Serialize, Deserialize };

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}

pub fn handle_keys(
    tcod: &mut Tcod,
    mut game: &mut Game,
    characters: &mut Vec<Character>,
    mut items: &mut HashMap<i32, Object>,
    mut player: &mut Character,
) -> PlayerAction {
    use tcod::input::KeyCode::*;
    use PlayerAction::*;

    let player_alive = player.object.alive;
    match (tcod.key, tcod.key.text(), player_alive) {
        // Movement keys
        ( Key { code: NumPad7, .. }, _, true) => {
            Object::player_move_or_attack(-1, -1, game, characters, &mut player.object);
            TookTurn
        },
        ( Key { code: NumPad8, .. }, _, true) => {
            Object::player_move_or_attack(0, -1, game, characters, &mut player.object);
            TookTurn
        },
        ( Key { code: NumPad9, .. }, _, true) => {
            Object::player_move_or_attack(1, -1, game, characters, &mut player.object);
            TookTurn
        },
        ( Key { code: NumPad4, .. }, _, true) => {
            Object::player_move_or_attack(-1, 0, game, characters, &mut player.object);
            TookTurn
        },
        ( Key { code: NumPad6, .. }, _, true) => {
            Object::player_move_or_attack(1, 0, game, characters, &mut player.object);
            TookTurn
        },
        ( Key { code: NumPad1, .. }, _, true) => {
            Object::player_move_or_attack(-1, 1, game, characters, &mut player.object);
            TookTurn
        },
        ( Key { code: NumPad2, .. }, _, true) => {
            Object::player_move_or_attack(0, 1, game, characters, &mut player.object);
            TookTurn
        },
        ( Key { code: NumPad3, .. }, _, true) => {
            Object::player_move_or_attack(1, 1, game, characters, &mut player.object);
            TookTurn
        },

        // Wait a turn
        ( Key { code: NumPad5, .. }, _, true) => {
            TookTurn // Wait a turn
        },

        ( Key { code: Text, .. }, "<", true) => {
            // Go down stairs if the player is on top of them.
            let player_on_stairs = items
                .values()
                .any(|obj| obj.pos() == player.object.pos() && obj.name == "Stairs");
            if player_on_stairs {
                next_level(tcod, game, &mut player.object, characters, items);
            }
            DidntTakeTurn
        }

        // Action keys
        // Grab the item at your position.
        ( Key { code: Text, .. }, "g", true) => {
            // Pick up an item
            let mut item_id: i32 = -1;

            for (key, item) in items.iter() {
                if (item.pos() == player.object.pos()) && (item.item.is_some()) {
                    item_id = *key
                }
            }

            if item_id > 0 {
                Object::pick_item_up(item_id, game, items, player);
                TookTurn
            } else {
                game.messages.add(
                    format!("There's no item to grab..."),
                    RED
                );
                DidntTakeTurn
            }
        },

        ( Key { code: Text, .. }, "i", true) => {
            // Show the inventory.
            let inventory_index = inventory_menu(
                player,
                "Press the key next to an item to use it, or any other to cancel.\n",
                &mut tcod.root
            );
            if let Some(inventory_index) = inventory_index {
                Object::use_item(inventory_index, tcod, game, characters, player, items);
            }
            TookTurn
        },

        ( Key { code: Text, .. }, "c", true) => {
            // Displays character information.
            let level = player.object.level;
            let level_up_xp = LEVEL_UP_BASE + player.object.level * LEVEL_UP_FACTOR;
            if let Some(fighter) = player.object.fighter.as_ref() {
                let msg = format!(
                    "Character information

Level: {}
Experience: {}
Experience to level up: {}

Maximum HP: {}
Attack: {}
Defense: {}",
                    level, fighter.exp, level_up_xp, fighter.max_hp, fighter.power, fighter.defense
                );
                msgbox(&msg, CHARACTER_SCREEN_WIDTH, &mut tcod.root);
            }
            DidntTakeTurn
        },

        ( Key { code: Text, .. }, "d", true) => {
            // Show the inventory. If an item is selected, drop it.
            let inventory_index = inventory_menu(
                &player, "Press a listed key to drop an item, or another key to cancel.\n",
                &mut tcod.root,
            );
            if let Some(inventory_index) = inventory_index {
                Object::drop_item(inventory_index, &mut game, &mut items, &mut player);
            }
            DidntTakeTurn
        },

/*        // DEBUG-KEYS
        ( Key { code: Text, .. }, "z", true) => { // Prints the list of items on the floor.
            let mut char_count = 0;
            for character in characters {
                if character.object.alive == true {
                    char_count += 1;
                }
            }
            println!("Items: {:?}\nCharacters: {:?}", items.len(), char_count);
            DidntTakeTurn
        },
        ( Key { code: Text, .. }, "x", true) => { // Prints the player's inventory
            println!("{:?}", player.inventory.as_ref());
            DidntTakeTurn
        },

        ( Key { code: Text, .. }, "c", true) => {
            // Go down stairs if the player is on top of them.
            next_level(tcod, game, &mut player.object, characters, items);
            DidntTakeTurn
        }

        ( Key { code: Text, .. }, "v", true) => {
            for character in characters {
                println!("{}", character.object.name);
                character.object.fighter.map(|f| println!("{:?}", f));
            }
            DidntTakeTurn
        }
*/
//      This code is temporarily removed, as it breaks the laptop on which it is being written.
//      Note: The fact that it breaks this specific laptop is proof that it functions correctly.
        ( Key { code: Enter, alt: true, .. }, _, _, ) => {
            // Alt+Enter: Toggles fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
            DidntTakeTurn
        },
        ( Key { code: Escape, .. }, _, _) => Exit, // Exits game

        _ => DidntTakeTurn,
    }
}
