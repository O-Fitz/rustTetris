
pub mod block;

const GRID_HIGHT: usize = 20;
const GRID_WIDTH: usize = 10;

pub struct Grid {
    arr: [[bool; GRID_WIDTH]; GRID_HIGHT],
}


impl Grid {

}

pub fn init_grid() -> Grid {
    let arr = [[false; GRID_WIDTH]; GRID_HIGHT];

    return Grid{arr};
}


