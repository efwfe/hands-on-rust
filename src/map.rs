// this file contains map module
// module scope have their own scope
// 1. default everything in module is private to that module.
// 2. make shared module entries public with pub keyword.

//  function: pub fn my_function()
//  structs: pub struct myStruct
//  enumerations: pub enum MyEnum
//  implemented functions: impl MyStruct{ pub fn my_func()}

//  make individual members of a structure public (they default to being private)
//  pub struct MyPublicStructure{
//      pub name: str,
//      private_age: int        
// }
use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType{
    Wall,
    Floor,
}
// clone makes deepcopy
// copy changes from one varibale to another, takes copy instead of moving
// PartialEq compare with == operator


// Create an empty Map

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map{
    pub fn new() ->Self{
        Self {
             tiles: vec![TileType::Floor; NUM_TILES]
        }
    }

    pub fn render(&self, ctx: &mut BTerm){
        for y in 0..SCREEN_HEIGHT{
            for x in 0..SCREEN_WIDTH{
                let idx = map_idx(x, y);
                match self.tiles[idx]{
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    },
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            }
        }
    }

}

// Index the Map
pub fn map_idx(x:i32, y:i32) -> usize{
    ((y * SCREEN_WIDTH) + x) as usize
}