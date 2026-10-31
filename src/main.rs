mod maze;

fn main() {
    let maze = maze::generate(15);

    let map = maze.into_map();
    display_map(&map);
}

fn display_map(map: &Vec<Vec<u8>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 1 {
                print!("\x1b[40m##");
            } else {
                print!("\x1b[107m  ");
            }
        }
        println!("\x1b[0m");
    }
}
