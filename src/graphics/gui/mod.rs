pub mod menu;

use crate::*;
use crate::objects::*;

pub fn render_gui(tcod: &mut Tcod, game: &Game, characters: &[Object], items: &HashMap<i32, Object>) {
    render_panel(tcod, game, characters, items);
}

fn render_panel(tcod: &mut Tcod, game: &Game, characters: &[Object], items: &HashMap<i32, Object>) {
    // Prepares the GUI panel.
    tcod.panel.set_default_background(BLACK);
    tcod.panel.clear();

    // Print the game messages, line by line.
    let mut y = MSG_HEIGHT as i32;
    for &(ref msg, color) in game.messages.iter().rev() {
        let msg_height = tcod.panel.get_height_rect(MSG_X, y, MSG_WIDTH, 0, msg);
        y -= msg_height;
        if y < 0 {
            break;
        }
        tcod.panel.set_default_foreground(color);
        tcod.panel.print_rect(MSG_X, y, MSG_WIDTH, 0, msg);
    }

    // Show the player's stats.
    let hp = game.player.fighter.unwrap().hp;
    let max_hp = game.player.fighter.unwrap().max_hp;
    render_bar(
        &mut tcod.panel,
        1,
        0,
        BAR_WIDTH,
        "HP",
        hp,
        max_hp,
        LIGHT_RED,
        DARKER_RED,
    );

    // Show the list of objects beneath the mouse.
    tcod.panel.set_default_foreground(LIGHT_GREY);
    tcod.panel.print_ex(
        1,
        1,
        BackgroundFlag::None,
        TextAlignment::Left,
        get_names_under_mouse(tcod.mouse, characters, items, &tcod.fov),
    );

    // Blit the contents of 'panel' to the root console.
    blit(
        &tcod.panel,
        (0, 0),
        (SCREEN_WIDTH, PANEL_HEIGHT),
        &mut tcod.root,
        (0, PANEL_Y),
        1.0,
        1.0,
    );
}

pub struct Messages {
    messages: Vec<(String, Color)>,
}

impl Messages {
    pub fn new() -> Messages {
        Messages { messages: vec![] }
    }

    // Add the new message as a tuple containing the text and the color.
    pub fn add<T: Into<String>>(&mut self, message: T, color: Color) {
        self.messages.push((message.into(), color));
    }

    // Creates a 'DoubleEndedIterator' over the messages.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &(String, Color)> {
        self.messages.iter()
    }
}

fn get_names_under_mouse(mouse: Mouse, characters: &[Object], items: &HashMap<i32, Object>, fov_map: &FovMap) -> String {
    let (x, y) = (mouse.cx as i32, mouse.cy as i32);
    let mut names = Vec::new();

    // Creates a list with the names of all characters at mouse's coordinates in FOV.
    let character_names = characters
        .iter()
        .filter(|obj| obj.pos() == (x, y) && fov_map.is_in_fov(obj.x, obj.y))
        .map(|obj| obj.name.clone())
        .collect::<Vec<_>>();

    // Adds items to vector first so they always appear at the top of the list.
    for (_, item) in items {
        if item.pos() == (x, y) && fov_map.is_in_fov(item.x, item.y) {
            names.push(item.name.clone());
        }
    }

    // Adds characters into the same vector as above.
    for character in character_names.iter() {
        names.push(character.clone());
    }

    // Concatenates the vector items into a string separated by new lines
    names.join("\n")
}

// Renders a bar of some sort.
fn render_bar(
    panel: &mut Offscreen,
    x: i32,
    y: i32,
    total_width: i32,
    name: &str,
    value: i32,
    maximum: i32,
    bar_color: Color,
    back_color: Color,)
    {
        // Calculates the width of the bar.
        let bar_width = (value as f32 / maximum as f32 * total_width as f32) as i32;

        // Renders background of bar.
        panel.set_default_background(back_color);
        panel.rect(x, y, total_width, 1, false, BackgroundFlag::Screen);

        // Renders the bar over the top of it.
        panel.set_default_background(bar_color);
        if bar_width > 0 {
            panel.rect(x, y, bar_width, 1, false, BackgroundFlag::Screen);
        }

        //Centered text with relevant values.
        panel.set_default_foreground(WHITE);
        panel.print_ex(
            x + total_width / 2,
            y,
            BackgroundFlag::None,
            TextAlignment::Center,
            &format!("{}: {}/{}", name, value, maximum),
        );
}
