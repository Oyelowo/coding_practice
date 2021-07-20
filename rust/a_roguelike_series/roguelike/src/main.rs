use tcod::{colors::*, console::*};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20; // frames-per-second maximum

struct Tcod {
    root: Root,
}

fn main() {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod Way")
        .init();

    let mut tcod = Tcod { root };

    tcod::system::set_fps(LIMIT_FPS);

    while !tcod.root.window_closed() {
        tcod.root.set_default_foreground(WHITE);
        // Then we clear the console of anything that we drew on the previous frame.
        tcod.root.clear();
        tcod.root.put_char(1, 1, '@', BackgroundFlag::None);
        tcod.root.flush();
        tcod.root.wait_for_keypress(true);
    }
}
