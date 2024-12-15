

enum BlockType {
    IBlock,
    JBlock,
    LBlock,
    SBlock,
    TBlock,
    OBlock,
}

impl BlockType {}


struct Block {
    grid: [[bool; 4]; 4],
    block_type: BlockType
}

impl Block {
    fn rotate_left(&mut self) {
        let mut newgrid = [[false; 4]; 4];
        for i in 0..4{
            for j in 0..4{
                newgrid[4-j][i] = self.grid[i][j];
            }
        }
        self.grid = newgrid;
    }

    fn rotate_right(&mut self) {
        let mut newgrid = [[false; 4]; 4];
        for i in 0..4{
            for j in 0..4{
                newgrid[j][4-i] = self.grid[i][j];
            }
        }
        self.grid = newgrid;
    }
}




