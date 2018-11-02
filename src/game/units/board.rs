use rand::{
    distributions::{Distribution, Standard},
    Rng,
    self,
};

use std::mem;

enum Shape {
    IBLOCK,
    LBLOCK,
    RLBLOCK,
    ZBLOCK,
    RZBLOCK,
    TBLOCK,
    SQBLOCK,
}

enum Rotation {
    NIN,
    ONE,
    TWO,
    THR,
}

enum Direction {
    DOWN,
    LEFT,
    RIGHT,
}

//for iterating over enum randomly
impl Distribution<Shape> for Standard{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Shape {
        match rng.gen_range(0, 6) {
            0 => Shape::IBLOCK,
            1 => Shape::LBLOCK,
            2 => Shape::RLBLOCK,
            3 => Shape::ZBLOCK,
            4 => Shape::RZBLOCK,
            5 => Shape::TBLOCK,
            6 => Shape::SQBLOCK,
            _ => panic!("no shape"),
        }
    }
}

struct Blocks {
    shape: Shape,
    color: (u8, u8, u8),
    block_pos: [(u8, u8); 4],
    rotation: Rotation,
}

struct Board {
    board: [[bool; 10]; 20],
    level: u8,
    score: u32,
    lines: u16,
    b_to_tetris: [u8; 10],
    next_block: Blocks,
}

impl Board {
    fn place(&self, pos: [(u8, u8); 4]){
        for a in &pos {
            let coords: (i32, i32) = ((super::MULTI_BOARD_OFFSET.0 + a.0 as i32 * super::TILE_SIZE),
                                    (super::MULTI_BOARD_OFFSET.1 + a.1 as i32 * super::TILE_SIZE) );
            //self.board[coords.0][coords.1] = true;
            //self.b_to_tetris[coords.1] += 1;
            //if self.b_to_tetris[coords.1] == 10 {
                //TETRIS!!!
                //self.b_to_tetris[coords.1] == 0
        }
    }

    fn can_move(&self, direction: Direction, block: Blocks) -> bool{
        match direction {
            Direction::DOWN => {
                if block.block_pos[0].1 == 19 { false }
                else if self.board[(block.block_pos[0].0) as usize][(block.block_pos[0].1 + 1) as usize] { false }
                else { true }
            },
            Direction::LEFT => {
                if block.block_pos[0].0 == 0 { false }
                else if self.board[(block.block_pos[0].0 - 1) as usize][(block.block_pos[0].1) as usize] { false }
                else { true }
            },
            Direction::RIGHT => {
                if block.block_pos[1].0 == 9 { false }
                else if self.board[(block.block_pos[1].0 + 1) as usize][(block.block_pos[0].1) as usize] { false }
                else { true }
            },
            _ => { false }
        }
    }

    fn spawn_block(&mut self) -> Blocks {
        let mut block: Blocks = mem::replace(&mut self.next_block, Blocks::new());
        block
        //block.block_pos = (
        //TODO: Update block pos from next window to coords
        //How do we store Next block coords?
    }
}

impl Blocks {
    fn new() -> Blocks {

        let block_shape: Shape = rand::random();

        match block_shape {
            Shape::RLBLOCK => { let block = Blocks { shape: block_shape, rotation: Rotation::NIN, color: (255, 100, 0), block_pos: [(0, 0), (0, 1), (0, 1), (2, 2)] }; return block; },

            Shape::SQBLOCK => { let block = Blocks { shape: block_shape, rotation: Rotation::NIN, color: (255, 255, 50), block_pos: [(0, 1), (0, 1), (0, 0), (1, 1)] }; return block; },

            Shape::LBLOCK => { let block = Blocks { shape: block_shape, rotation: Rotation::NIN, color: (0, 0, 255), block_pos: [(0, 1), (1, 1), (2, 2), (1, 0)] }; return block; },

            Shape::ZBLOCK => { let block = Blocks { shape: block_shape, rotation: Rotation::NIN, color: (255, 0, 0), block_pos: [(0, 1), (1, 2), (1, 1), (0, 0)] }; return block; },

            Shape::RZBLOCK => { let block = Blocks { shape: block_shape, rotation: Rotation::NIN, color: (0, 255, 0), block_pos: [(0, 1), (1, 2), (0, 0), (1, 1)] }; return block; },

            Shape::TBLOCK => { let block = Blocks { shape: block_shape, rotation: Rotation::NIN, color: (255, 0, 255), block_pos: [(0, 1), (1, 2), (1, 1), (0, 1)] }; return block; },

            Shape::IBLOCK => { let block = Blocks { shape: block_shape, rotation: Rotation::NIN, color: (0, 255, 255), block_pos: [(1, 1), (1, 1), (0, 1), (2, 3)] }; return block; },

            _ => { panic!("blockshape error") },
        };
    }
}

