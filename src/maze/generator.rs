use std::num::{NonZero, NonZeroUsize};

use rand::seq::IndexedRandom;

use super::Maze;

pub fn generate(n: usize) -> Maze {
    let mut maze = Maze::new(n);

    let mut sets = (1..n + 1).collect::<Vec<usize>>();
    let mut curr_row = 0;

    for _ in 0..n - 1 {
        println!("{:?}", sets);
        for i in 0..n - 1 {
            if sets[i + 1] != sets[i] && rand::random_bool(0.8) {
                let change = sets[i + 1];

                break_wall(i, curr_row, n, true, false, &mut maze);

                for j in 0..n {
                    if sets[j] == change {
                        sets[j] = sets[i]
                    }
                }
            }
        }
        println!("{:?}", sets);

        let mut vertical: Vec<Option<NonZeroUsize>> = vec![None; n];

        for i in 1..n + 1 {
            if sets.contains(&i) {
                loop {
                    let indices = get_indices(&sets, i);
                    let index = indices.choose(&mut rand::rng()).unwrap();
                    vertical[*index] = NonZero::new(sets[*index]);
                    break_wall(*index, curr_row, n, false, true, &mut maze);
                    if rand::random_bool(0.7) {
                        break;
                    }
                }
            }
        }

        for i in 0..n {
            match vertical[i] {
                Some(val) => sets[i] = usize::from(val),
                None => {
                    for j in 1..n + 1 {
                        if !sets.contains(&j) {
                            sets[i] = j;
                            break;
                        }
                    }
                }
            }
        }
        curr_row += 1;
    }

    for i in 0..n - 1 {
        if sets[i + 1] != sets[i] {
            break_wall(i, curr_row, n, true, false, &mut maze);
            sets[i + 1] = sets[i];
        }
    }
    generate_cells(&mut maze);

    maze
}

fn get_indices(sets: &Vec<usize>, n: usize) -> Vec<usize> {
    let mut indices = Vec::new();
    for i in 0..sets.len() {
        if sets[i] == n {
            indices.push(i);
        }
    }
    indices
}

fn break_wall(i: usize, row: usize, width: usize, right: bool, bottom: bool, maze: &mut Maze) {
    if right {
        maze.cells[row * width + i].right = false;
    }
    if bottom {
        maze.cells[row * width + i].down = false;
    }
}

fn generate_cells(maze: &mut Maze) {
    for y in 0..maze.n {
        for x in 0..maze.n {
            let index = x + y * maze.n;
            if x != 0 {
                maze.cells[index].left = maze.cells[index - 1].right;
            }
            if y != 0 {
                maze.cells[index].up = maze.cells[index - maze.n].down;
            }
        }
    }
}
