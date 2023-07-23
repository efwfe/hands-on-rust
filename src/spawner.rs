
use crate::prelude::*;

// mutable reference to the world and the location to spawn the player
pub fn spawn_player(ecs: &mut World, pos: Point){
    // calling push creates a new entity composed of the listed components
    ecs.push(
        (
            Player, 
            pos,
             Render{
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@')
        })
    );
}

