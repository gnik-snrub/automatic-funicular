pub mod tiles;
pub mod dungeon;

use crate::graphics::gui::Messages;
use crate::objects::Object;
use crate::graphics::gen_colors;
use tiles::Tile;
use dungeon::*;

use std::collections::HashMap;
use rand::*;

use tcod::map::FovAlgorithm;

// Determines Field-Of-View
pub const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic; // Default FOV Algorithm
pub const FOV_LIGHT_WALLS: bool = true;
pub const TORCH_RADIUS: i32 = 10;

// Size of the map
pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 43;

// Dungeon room limitations
const ROOM_MAX_SIZE: i32 = 12;
const ROOM_MIN_SIZE: i32 = 4;
const MAX_ROOMS: i32 = 18;
const MAX_ROOM_MONSTERS: i32 = 3;
const MAX_ROOM_ITEMS: i32 = 2;

pub type Map = Vec<Vec<Tile>>;

pub struct Game {
    pub map: Map,
    pub messages: Messages,
    pub player: Object,
}

impl Game {
    pub fn new(mut characters: &mut Vec<Object>, mut items: &mut HashMap<i32, Object>) -> Game {
        let mut player = Object::player();
        let map = make_map(&mut player, &mut characters, &mut items);
        Game {
            map: map,
            messages: Messages::new(),
            player: player,
        }
    }
}


pub fn make_map(player: &mut Object, characters: &mut Vec<Object>, items: &mut HashMap<i32, Object>) -> Map {
    // Generate dungeon floor colors alongside variation
    let colors = gen_colors();

    // Fill map with wall tiles
    let mut map = vec![vec![Tile::wall(&colors); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    //Creates vector to store rooms
    let mut rooms: Vec<Rect> = vec![];

    // Determines dungeon gen path to follow
    // <0.5 should appear more ruinous
    // >0.5 should appear more designed, and constructed
    let world_path = rand::random::<f32>();
    println!("World gen = {}", world_path);

    let mut item_counter = 1;

    for _ in 0..MAX_ROOMS {
        // Random width and height
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        // Random position without going outside the map boundaries
        let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
        let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);

        let new_room = Rect::new(x, y, w, h);

        // Run through the other rooms and see if they interact with this one`
        let failed = rooms.iter().any(|other_room| new_room.intersects_with(other_room));

        // Adds in rooms according to world path value
        if (world_path < 0.5 && (!failed || failed)) || (world_path > 0.5 && !failed) {
            // Paints room onto map tiles
            create_room(new_room, &mut map, &colors);
            place_characters(new_room, &map, characters);
            place_items(new_room, items, &mut item_counter);

            // Center coordinates of the new room, will be used later
            let (new_x, new_y) = new_room.center();

            if rooms.is_empty() {

                // This is the first room, where the player starts at
                player.set_pos(new_x, new_y);

            } else {

                // All rooms after the first connect to the previous room with a tunnel
                // Center coordinates of the previous room
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                // Arbitrarily decides to begin with either a vertical, or horizontal tunnel
                if rand::random() {
                    // Horizontal tunnel first
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map, &colors);
                    create_v_tunnel(prev_y, new_y, new_x, &mut map, &colors);
                } else {
                    // Vertical tunnel first
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map, &colors);
                    create_h_tunnel(prev_x, new_x, new_y, &mut map, &colors);
                }
            }
        }

        rooms.push(new_room)
    }

    map
}

fn place_characters(room: Rect, map: &Map, characters: &mut Vec<Object>) {
    // Choose random number of monsters
    let num_monsters = rand::thread_rng().gen_range(0, MAX_ROOM_MONSTERS + 1);

    for _ in 0..num_monsters {
        // Choose random spot for the monster
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        if !Object::is_blocked(x, y, map, characters) {
            let mut monster = if rand::random::<f32>() < 0.8 {
                Object::fire_elemental(x, y)
            } else {
                Object::crystal_lizard(x, y)
            };
            monster.alive = true;
            characters.push(monster);
        }
    }
}

fn place_items(room: Rect, items: &mut HashMap<i32, Object>, item_counter: &mut i32) {
    let num_items = rand::thread_rng().gen_range(0, MAX_ROOM_ITEMS + 1);

    for _ in 0..num_items {
        // Select random spot for the item.
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        // Create a health potion.
        let item = Object::health_pot(x, y);
        items.insert(*item_counter, item);
        *item_counter += 1;
        println!("{:?}", items.len());
    }
    println!("{:?}", items);
}

// --- TO-DO ---
// No-Dead end algorithm:
// - Check to see if there are at least 2 empty tiles connected to a tile
// - If there is not at least 2, scan the map
// - Check each tile for distance away from the tile lacking connections
// - Find the tile with the shortest distance
// - Run the same algorithm to connect tunnels between them
