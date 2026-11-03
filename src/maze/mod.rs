mod cell;
mod generator;

pub use generator::generate;

pub struct Maze {
    pub n: usize,
    pub cells: Vec<cell::MazeCell>,
}

impl Maze {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            cells: vec![cell::MazeCell::new(); n * n],
        }
    }

    pub fn into_map(&self) -> Vec<Vec<u8>> {
        let width = self.n * 3;
        let height = self.n * 3;

        let mut map = vec![vec![1u8; width]; height];

        for y in 0..self.n {
            for x in 0..self.n {
                let cell = self.cells[x + y * self.n];

                let mx = x * 3;
                let my = y * 3;

                map[my + 1][mx + 1] = 0;

                if !cell.up {
                    map[my][mx + 1] = 0;
                }

                if !cell.left {
                    map[my + 1][mx] = 0;
                }

                if !cell.right {
                    map[my + 1][mx + 2] = 0;
                }

                if !cell.down {
                    map[my + 2][mx + 1] = 0;
                }
            }
        }

        loop {
            let x = rand::random_range(0..self.n * 3);
            let y = rand::random_range(0..self.n * 3);

            if map[y][x] == 0 {
                map[y][x] = 2;
                break;
            }
        }

        map
    }
}
