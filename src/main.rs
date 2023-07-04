use bracket_lib::prelude::*;

// Game state
struct State {}

// Game Modes
enum GameMode{
    Menu,
    Playing,
    End,
}

impl GameState for State{
    fn tick(&mut self, ctx: &mut BTerm){
        ctx.cls();
        ctx.print(1, 1, "Hello World!");
    }
}
fn main() -> BError {
   let context = BTermBuilder::simple80x50()
    .with_title("Flappy Dragon")
    .build()?;

    main_loop(context, State{})

}
