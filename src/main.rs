use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

// Game state
struct State {
    mode: GameMode,
    frame_time:f32,
    player:Player,
}

impl State{
    fn new()->Self{
        State {
             mode: GameMode::Menu, 
             frame_time: 0.0,
             player: Player::new(5, 25)
        }
    }
}

impl State{
    // Play function stub
    fn play(&mut self, ctx: &mut BTerm){
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key{
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0,0, "Press SPACE to flap");
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }

    fn restart(&mut self){
        self.player = Player::new(5, 35);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        
    }

    // Main Menu
    fn main_menu(&mut self, ctx: &mut BTerm){
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key{
            match key{
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ =>{}
            }
        }
    }

    // Game Over 
    fn dead(&mut self, ctx:&mut BTerm){
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");
        if let Some(key) = ctx.key{
            match key{
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ =>{}
            }
        }
    }
}

struct Player{
    x:i32, // x position of the player
    y:i32,
    velocity: f32,
}

impl Player{
    fn new(x: i32, y:i32)->Self{
        Self{
            x,
            y,
            velocity: 0.0
        }
    }

    fn render(&mut self, ctx: &mut BTerm){
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'))
    }

    fn gravity_and_move(&mut self){
        if self.velocity < 2.0 {
            self.velocity+=0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self){
        self.velocity -= 2.0;
    }

}

// Game Modes
enum GameMode{
    Menu,
    Playing,
    End,
}


impl GameState for State{
    fn tick(&mut self, ctx: &mut BTerm){
        match self.mode{
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx ),
            GameMode::Playing => self.play(ctx),
        }
    }
}


fn main() -> BError {
   let context = BTermBuilder::simple80x50()
    .with_title("Flappy Dragon")
    .build()?;

    main_loop(context, State::new())

}
