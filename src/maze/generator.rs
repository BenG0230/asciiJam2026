use std::num::{NonZero, NonZeroUsize};

use rand::seq::IndexedRandom;

use super::Maze;

pub fn generate(n: usize) -> Maze {
    let mut maze = Maze::new(n);

    let mut sets = (1..n + 1).collect::<Vec<usize>>();
    let mut curr_row = 0;

    for _ in 0..n - 1 {
        for i in 0..n - 1 {
            if sets[i + 1] != sets[i] && rand::random_bool(0.7) {
                let change = sets[i + 1];

                for j in 0..n {
                    if sets[j] == change {
                        sets[j] = sets[i]
                    }
                }
            }
        }

        let mut vertical: Vec<Option<NonZeroUsize>> = vec![None; n];

        for i in 1..n + 1 {
            if sets.contains(&i) {
                loop {
                    let indices = get_indices(&sets, i);
                    let n = indices.choose(&mut rand::rng()).unwrap();
                    vertical[*n] = NonZero::new(sets[*n]);
                    if rand::random_bool(0.9) {
                        break;
                    }
                }
            }
        }
        generate_cells(&sets, &vertical, curr_row, &mut maze);

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
            sets[i + 1] = sets[i];
        }
    }

    generate_cells(&sets, &vec![None; n], curr_row, &mut maze);

    maze.display();

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

fn generate_cells(
    sets: &Vec<usize>,
    vertical: &Vec<Option<NonZeroUsize>>,
    row: usize,
    maze: &mut Maze,
) {
    println!();
    for i in 0..sets.len() {
        // check right
        if i > 0 && sets[i - 1] == sets[i] {
            maze.cells[row * sets.len() + i].left = false;
        }

        if i < sets.len() - 1 && sets[i + 1] == sets[i] {
            maze.cells[row * sets.len() + i].right = false;
        }

        if let Some(_) = vertical[i] {
            maze.cells[row * sets.len() + i].down = false;
            maze.cells[(row + 1) * sets.len() + i].up = false;
        }

        println!("{}", maze.cells[row * sets.len() + i]);
    }
}
