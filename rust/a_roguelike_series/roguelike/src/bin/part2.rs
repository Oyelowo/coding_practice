use tcod::{
    colors,
    console::{Offscreen, Root},
    BackgroundFlag, Color, Console,
};

const SCREEN_WIDTH: u16 = 640;
const SCREEN_HEIGHT: u16 = 480;

struct Tcod {
    root: Root,
    con: Offscreen,
}

fn main() {
    let con = Offscreen::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
    let tcod = Tcod { root, con };
    tcod.con.set_default_foreground(colors::WHITE);
    tcod.con.clear();
    tcod.con
        .put_char(player_x, player_y, '@', BackgroundFlag::None);
}
