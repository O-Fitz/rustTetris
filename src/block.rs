
use rand::{Rng};

pub enum BlockType {
    IBlock,
    JBlock,
    LBlock,
    SBlock,
    ZBlock,
    TBlock,
    OBlock,
}

pub struct Block {
    grid: [[bool; 4]; 4],
    block_type: BlockType
}

impl Block {
    pub fn rotate_left(&mut self) {
        let mut newgrid = [[false; 4]; 4];
        for i in 0..4{
            for j in 0..4{
                newgrid[4-j][i] = self.grid[i][j];
            }
        }
        self.grid = newgrid;
    }

    pub fn rotate_right(&mut self) {
        let mut newgrid = [[false; 4]; 4];
        for i in 0..4{
            for j in 0..4{
                newgrid[j][4-i] = self.grid[i][j];
            }
        }
        self.grid = newgrid;
    }
}


pub fn make_i() -> Block {

    let mut grid = [[false; 4]; 4];

    grid[1][0] = true;
    grid[1][1] = true;
    grid[1][2] = true;
    grid[1][3] = true;

    return Block{grid, block_type:BlockType::IBlock};
}


pub fn make_j() -> Block {

    let mut grid = [[false; 4]; 4];

    grid[2][0] = true;
    grid[2][1] = true;
    grid[2][2] = true;
    grid[1][2] = true;

    return Block{grid, block_type:BlockType::IBlock};
}

pub fn make_o() -> Block {

    let mut grid = [[false; 4]; 4];

    grid[1][1] = true;
    grid[1][2] = true;
    grid[2][1] = true;
    grid[2][2] = true;

    return Block{grid, block_type:BlockType::IBlock};
}

pub fn make_t() -> Block {

    let mut grid = [[false; 4]; 4];

    grid[1][1] = true;
    grid[2][1] = true;
    grid[3][1] = true;
    grid[2][2] = true;

    return Block{grid, block_type:BlockType::IBlock};
}

pub fn make_s() -> Block {

    let mut grid = [[false; 4]; 4];

    grid[2][1] = true;
    grid[3][1] = true;
    grid[1][2] = true;
    grid[2][2] = true;

    return Block{grid, block_type:BlockType::IBlock};
}

pub fn make_z() -> Block {

    let mut grid = [[false; 4]; 4];

    grid[1][1] = true;
    grid[2][1] = true;
    grid[2][2] = true;
    grid[3][2] = true;

    return Block{grid, block_type:BlockType::IBlock};
}

pub fn make_l() -> Block {

    let mut grid = [[false; 4]; 4];

    grid[1][0] = true;
    grid[1][1] = true;
    grid[1][2] = true;
    grid[2][2] = true;

    return Block{grid, block_type:BlockType::IBlock};
}

pub fn make_random_block() -> Block {
    let mut rng = rand::thread_rng();

    match rng.gen_range(0..7) {
        0 => make_i(),
        1 => make_o(),
        2 => make_j(),
        3 => make_l(),
        4 => make_s(),
        5 => make_z(),
        6 => make_t(),
        _ => make_i()
    }
}
