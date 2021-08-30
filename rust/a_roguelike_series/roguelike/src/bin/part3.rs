use std::cmp;

use tcod::{
    colors::{self, WHITE},
    console::{blit, Offscreen, Root},
    BackgroundFlag, Color, Console, FontLayout, FontType,
};

const SCREEN_WIDTH: i32 = 90;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20; // 20 frames-per-second maximum

// size of the map
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};

/// A tile of the map and its properties
#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Self {
            blocked: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Self {
            blocked: true,
            block_sight: true,
        }
    }
}

struct Tcod {
    root: Root,
    con: Offscreen,
}

/// This is a generic object: the player, a monster, an item, the stairs...
/// It's always represented by a character on screen.
#[derive(Debug)]
struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object { x, y, char, color }
    }

    /// move by the given amount
    pub fn move_by(&mut self, dx: i32, dy: i32, game: &Game) {
        if !game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }

    /// set the color and then draw the character that represents this object at its position
    pub fn draw(&self, console: &mut dyn Console) {
        console.set_default_foreground(self.color);
        console.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

type Map = Vec<Vec<Tile>>;

struct Game {
    map: Map,
}

impl Game {
    fn new(map: Map) -> Self {
        Self { map }
    }
}

/// A rectangle on the map, used to characterise a room.
#[derive(Clone, Copy, Debug)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }
}

/*
The +1 business here is a bit subtle: the A..B notation specifies a range that’s inclusive at the beginning but exclusive at the end. For example 1..5 represents numbers 1, 2, 3 and 4 but not 5.

So to go through all the values between x1 and x2 (including both), we’d have to write x1..(x2 + 1). But we want to make sure each room is enclosed in a wall, so we want to go from x1 to x2 exclusive. To do that, we add 1 to the first coordinate and subtract one from the second, ending up with (x1 + 1)..x2. If x1 is 1 and x2 is 5, we would put empty tiles at positions 2, 3 and 4 and leave 1 and 5 solid.
*/

fn create_room(room: Rect, map: &mut Map) {
    // go through the tiles in the rectangle and make them passable
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    // horizontal tunnel. `min()` and `max()` are used in case `x1 > x2`
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    // vertical tunnel
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

/*
    There’s a ton of different ways to create the map. One common alternative is one continuous Vec with MAP_HEIGHT * MAP_WIDTH items. To access a tile on (x, y), you would do map[y * MAP_WIDTH + x]. The advantage is that you only do one array lookup instead of two and iterating over every object in the map is faster because they’re all in the same region of memory.
Or you could treat walls and everything else in the map as just another Object and store them there. This would make the game structure simpler (everything is an Object) and more flexible (just add HP to make a wall destructible, or damage to one that’s supposed to be covered with spikes).
*/
fn make_map() -> Map {
    // fill map with "unblocked" tiles
    let mut map: Map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    /*     map[30][22] = Tile::wall(); */
    /*     map[50][22] = Tile::wall(); */
    /*     map[30][22].blocked = true; */

    // create two rooms
    let room1 = Rect::new(20, 15, 10, 15);
    let room2 = Rect::new(50, 15, 10, 15);
    create_room(room1, &mut map);
    create_room(room2, &mut map);
    create_h_tunnel(25, 55, 23, &mut map);
    map
}

fn handle_keys(tcod: &mut Tcod, game: &Game, player: &mut Object) -> bool {
    use tcod::input::{Key, KeyCode::*};

    let key = tcod.root.wait_for_keypress(true);

    match key {
        // movement keys
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,
        Key { code: Up, .. } => player.move_by(0, -1, game),
        Key { code: Down, .. } => player.move_by(0, 1, game),
        Key { code: Left, .. } => player.move_by(-1, 0, game),
        Key { code: Right, .. } => player.move_by(1, 0, game),
        _ => {}
    }
    false
}

fn render_all(tcod: &mut Tcod, game: &Game, objects: &[Object]) {
    // draw all objects in the list
    // objects.iter().for_each(|object| object.draw(&mut tcod.con));
    for object in objects {
        object.draw(&mut tcod.con);
    }

    // go through all the tiles in the map and set their background color
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = game.map[x as usize][y as usize].block_sight;

            if wall {
                tcod.con
                    .set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                tcod.con
                    .set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }

    // blit the contents of "con" to the root console
    blit(
        &tcod.con,
        (0, 0),
        (MAP_WIDTH, MAP_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );
}

fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod Way")
        .init();

    let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
    let mut tcod = Tcod { root, con };

    // create object representing the player
    // let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', colors::WHITE);
    let player = Object::new(25, 23, '@', colors::WHITE);

    // create an NPC
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', colors::YELLOW);

    // the list of objects with those two
    let mut objects = [player, npc];

    // place two pillars to test the map
    let game: Game = Game::new(make_map());

    while !tcod.root.window_closed() {
        // Then we clear the console of anything that we drew on the previous frame.
        tcod.con.clear();

        for object in &objects {
            object.draw(&mut tcod.con);
        }

        // blit the contents of "con" to the root console and present it
        /*   blit(
            &tcod.con,
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut tcod.root,
            (0, 0),
            1.0,
            1.0,
        ); */

        // render the screen
        render_all(&mut tcod, &game, &objects);

        tcod.root.flush();

        // handle keys and exit game if needed
        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, &game, player);

        if exit {
            break;
        }
    }
}
