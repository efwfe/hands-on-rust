
// Rust need to know that you want to include new module in programe compile
mod map; 
// imports map module into the global scope and set up the map::prefix
// within main.rs items declared in map may be accessed as map::my_function
mod camera;
mod player;
mod map_builder;
mod components;
mod spawner;
mod systems;

mod prelude{
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub const SCREEN_WIDTH: i32  = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    //  pub use super::map::*;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
}

use prelude::*;

struct State{
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State{
    fn new() -> Self{
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuiler::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        Self { ecs: ecs, resources: resources, systems: build_scheduler() }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.resources.insert(ctx.key);
        // TODO: execute systes
        self.systems.execute(&mut self.ecs, &mut self.resources);
        // TODO: render Draw Buffer
    }
}

fn main() -> BError{
    let context = BTermBuilder::simple80x50()
                        .with_title("Dungeon Crawler")
                        .with_fps_cap(30.0)
                        .with_dimensions(32, 32)
                        .with_tile_dimensions(32, 32)
                        .with_resource_path("resources/")
                        .with_font("dungeonfont.png", 32, 32)
                        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
                        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
                        .build()?;
    main_loop(context, State::new())
                        
}


