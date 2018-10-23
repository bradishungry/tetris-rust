extern crate rand;
use units;
use rand::Rng;

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

struct Blocks {
    shape: Shape,
    color: (u8, u8, u8),
    block_pos: [(i32, i32); 4],
    rotation: Rotation,
}

struct Board {
    board: [[bool; 10]; 20],
    level: u8;
    score: u32;
    lines: u16;
    b_to_tetris: [u8; 10],
    next_block: Blocks,
}

impl Board {
    fn place(&self, pos: [(i32, i32)]){
        for a in pos {
            let coords: (u8, u8) = |a| ((units::MULTI_BOARD_OFFSET.0 - a.0 / units::TILE_SIZE, units::MULTI_BOARD_OFFSET.1 - a.1 / units::TILE_SIZE));
            board[coords.0][coords.1] = true;
            b_to_tetris[coords.1]++;
            if b_to_tetris[coords.1] == 10 {
                //TETRIS!!!
                b_to_tetris[coords.1] == 0
            }
        }
    }

    fn can_move(&self, direction: Direction, block: Block) -> bool{
        match direction {
            Direction::DOWN => {
                if block.block_pos[0].1 == 19 { false }
                else if self.board[block.block_pos[0].0][block.block_pos[0].1 + 1] { false }
                else { true }
            },
            Direction::LEFT => {
                if block.block_pos[0].0 == 0 { false }
                else if self.board[block.block_pos[0].0 - 1][block.block_pos[0].1] { false }
                else { true }
            },
            Direction::RIGHT => {
                if block.block_pos[1].0 == 9 { false }
                else if self.board[block.block_pos[1].0 + 1][block.block_pos[0].1] { false }
                else { true }
            },
            _ => { false }
        };
    }

    fn spawn_block(&self) -> Blocks {
        let mut block = Ok(std::mem::replace(&mut self.next_block, Blocks::new())).unwrap();
        //block.block_pos = (
        //TODO: Update block pos from next window to coords
        //How do we store Next block coords?
    }
}

impl Blocks {
    fn new() -> Blocks {

        block_shape: Shape = rand::thread_rng().gen_range(0, 6);

        match block_shape {
            Shape::RLBLOCK => { let block = Blocks { color: (255, 100, 0), block_pos: (0, 0, 0, 1, 0, 1, 2, 2) },

            Shape::SQBLOCK => { let block = Blocks { color: (255, 255, 50), block_pos: (0, 1, 0, 1, 0, 0, 1, 1) } },

            Shape::LBLOCK => { let block = Blocks { color: (0, 0, 255), block_pos: (0, 1, 1, 1, 2, 2, 1, 0) } },

            Shape::ZBLOCK => { let block = Blocks { color: (255, 0, 0), block_pos: (0, 1, 1, 2, 1, 1, 0, 0) } },

            Shape::RZBLOCK => { let block = Blocks { color: (0, 255, 0), block_pos: (0, 1, 1, 2, 0, 0, 1, 1) } },

            Shape::TBLOCK => { let block = Blocks { color: (255, 0, 255), block_pos: (0, 1, 1, 2, 1, 1, 0, 1) } },

            Shape::IBLOCK => { let block = Blocks { color: (0, 255, 255), block_pos: (1, 1, 1, 1, 0, 1, 2, 3) } },

            _ => println!("NAH"),
        };
    }
}

