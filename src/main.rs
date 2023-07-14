
// Rust need to know that you want to include new module in programe compile
mod map; 
// imports map module into the global scope and set up the map::prefix
// within main.rs items declared in map may be accessed as map::my_function

mod prelude{
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32  = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    //  pub use super::map::*;
    pub use crate::map::*;
}

use prelude::*;

struct State{
    map: Map,
}

impl State{
    fn new() -> Self{
        Self{map: Map::new()}
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
    }
}

fn main() -> BError{
    let context = BTermBuilder::simple80x50()
                        .with_title("Dungeon Crawler")
                        .with_fps_cap(30.0)
                        .build()?;
    main_loop(context, State::new())
                        
}


