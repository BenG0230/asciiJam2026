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

    pub fn display(&self) {
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

        println!("\x1b[40m{}\x1b[0m", String::from("#").repeat(width * 2 + 2));
        for y in 0..height {
            print!("\x1b[40m#");
            for x in 0..width {
                if map[y][x] == 1 {
                    print!("\x1b[40m##");
                } else {
                    print!("\x1b[107m  ");
                }
            }
            println!("#\x1b[0m");
        }
        println!("\x1b[40m{}\x1b[0m", String::from("#").repeat(width * 2 + 2));
    }
}
