pub mod cnf_generator;

use cnf_generator::{at_most_k, exactly_k, mineguess};
use itertools::enumerate;
use rand::Rng;
use splr::*;

fn gen_map(x: i32, y: i32, bomb_num: i32) -> Vec<Vec<i32>> {
    let mut map = Vec::new();
    for i in 0..x {
        let mut row = Vec::new();
        for j in 0..y {
            row.push(0);
        }
        map.push(row);
    }
    for _ in 0..bomb_num {
        let mut nx = rand::rng().random_range(0..x) as usize;
        let mut ny = rand::rng().random_range(0..y) as usize;
        while map[nx][ny] == 1 {
            nx = rand::rng().random_range(0..x) as usize;
            ny = rand::rng().random_range(0..y) as usize;
        }
        map[nx][ny] = 1;
    }
    map
}
#[derive(Clone)]
struct Cell {
    x: usize,
    y: usize,
    is_bomb: bool,
    is_question: bool,
    is_flagged: bool,
    is_opened: bool,
    adjacent_bombs: i32,
}
#[derive(Clone)]
struct PlayMap {
    cells: Vec<Vec<Cell>>,
}

fn count_adjacent_bombs(map: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let mut count = 0;
    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            }
            let nx = x as i32 + i;
            let ny = y as i32 + j;
            if nx < 0 || ny < 0 {
                continue;
            }
            if nx >= map.len() as i32 || ny >= map[0].len() as i32 {
                continue;
            }
            if map[nx as usize][ny as usize] == 1 {
                count += 1;
            }
        }
    }
    count
}

fn get_adjacent_cells(map: &PlayMap, x: usize, y: usize) -> Vec<i32> {
    let mut cells = Vec::new();
    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            }
            let nx = x as i32 + i;
            let ny = y as i32 + j;
            if nx < 0 || ny < 0 {
                continue;
            }
            if nx >= map.cells.len() as i32 || ny >= map.cells[0].len() as i32 {
                continue;
            }
            cells.push(nx * map.cells[0].len() as i32 + ny + 1);
        }
    }
    cells
}

fn print_map(map: &PlayMap) {
    for row in map.cells.iter() {
        for cell in row.iter() {
            if cell.is_opened {
                if cell.is_bomb {
                    print!("* ");
                } else if cell.is_question {
                    print!("? ");
                } else {
                    print!("{} ", cell.adjacent_bombs);
                }
            } else {
                print!("- ");
            }
        }
        println!();
    }
}

fn can_clear(map: &PlayMap, x: usize, y: usize) -> bool {
    let mut map = map.clone();
    let mut rules: Vec<Vec<i32>> = Vec::new();
    let mut next_var = (x * y + 1) as i32;
    let config = splr::Config {
        splr_interface: true,
        quiet_mode: false,
        ..Default::default()
    };
    for row in map.cells.iter() {
        for cell in row.iter() {
            if cell.is_opened {
                if cell.is_bomb {
                    rules.push(vec![(cell.x * y + cell.y + 1) as i32]);
                } else if cell.is_question {
                    rules.push(vec![-((cell.x * y + cell.y + 1) as i32)]);
                } else if cell.adjacent_bombs == 0 {
                    rules.push(vec![-((cell.x * y + cell.y + 1) as i32)]);
                    let adjacent_cells = get_adjacent_cells(&map, cell.x, cell.y);
                    for adjcell in adjacent_cells {
                        rules.push(vec![-adjcell]);
                    }
                } else {
                    let adjacent_cells = get_adjacent_cells(&map, cell.x, cell.y);
                    // println!("x: {}, y: {}", cell.x, cell.y);
                    // println!("adjacent_cells: {:?}", adjacent_cells);
                    rules.push(vec![-((cell.x * y + cell.y + 1) as i32)]);
                    rules.extend(exactly_k(
                        &adjacent_cells,
                        cell.adjacent_bombs as usize,
                        &mut next_var,
                    ));
                }
            }
        }
    }
    // println!("rules: {:?}", rules);
    let mut solver = Solver::try_from((config, rules.as_ref())).expect("panic");

    let sure_model = mineguess(&mut solver, (x * y) as usize);
    println!("sure_model: {:?}", sure_model);
    let mut clearflag = true;
    let mut changed = false;
    for row in map.cells.iter_mut() {
        for cell in row.iter_mut() {
            if sure_model[cell.x * y + cell.y] != 0 && cell.is_opened == false {
                cell.is_opened = true;
                changed = true;
            }
            if cell.is_opened == false {
                clearflag = false;
            }
        }
    }
    if changed == false {
        return false;
    }
    if clearflag {
        println!("can clear!!");
        return true;
    }
    can_clear(&map, x, y)
}

fn main() {
    let map = gen_map(5, 5, 10);
    println!("{:?}", map);
    let mut visible_map: PlayMap = PlayMap {
        cells: map
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .map(|(y, cell)| Cell {
                        x,
                        y,
                        is_bomb: *cell == 1,
                        is_question: false,
                        is_flagged: false,
                        is_opened: *cell == 0,
                        adjacent_bombs: count_adjacent_bombs(&map, x, y),
                    })
                    .collect()
            })
            .collect(),
    };

    // visible_map.cells[0][0].is_opened = false;
    print_map(&visible_map);
    for i in 0..25 {
        let x = i % 5;
        let y = i / 5;
        if visible_map.cells[x][y].is_opened == false {
            continue;
        }
        visible_map.cells[x][y].is_opened = false;
        print_map(&visible_map);
        if can_clear(&visible_map, 5, 5) {
            println!("can clear!!");
            println!();
            println!();
        } else {
            println!("can't clear!!");
            println!();
            println!();
            visible_map.cells[x][y].is_opened = true;
        }
    }
    print_map(&visible_map);
    for i in 0..25 {
        let x = i % 5;
        let y = i / 5;
        if visible_map.cells[x][y].is_bomb == true {
            continue;
        }
        visible_map.cells[x][y].is_question = true;
        print_map(&visible_map);
        if can_clear(&visible_map, 5, 5) {
            println!("can clear!!");
        } else {
            println!("can't clear!!");
            visible_map.cells[x][y].is_question = false;
        }
    }
    println!("last map");
    print_map(&visible_map);
    for row in visible_map.cells.iter_mut() {
        for cell in row.iter_mut() {
            cell.is_opened = true;
        }
    }
    print_map(&visible_map);

    // let mut rules: Vec<Vec<i32>> = Vec::new();

    // let config = splr::Config {
    //     splr_interface: true,
    //     quiet_mode: false,
    //     ..Default::default()
    // };
    // let mut next_var = 10;
    // rules.extend(exactly_k(&vec![2, 5, 4], 1, &mut next_var));
    // rules.extend(exactly_k(&vec![2, 5, 6], 1, &mut next_var));
    // rules.extend(exactly_k(&vec![1, 2, 5, 8, 7], 1, &mut next_var));
    // rules.extend(exactly_k(&vec![1, 2, 3, 4, 6, 7, 8, 9], 2, &mut next_var));
    // rules.extend(exactly_k(&vec![2, 3, 5, 8, 9], 2, &mut next_var));
    // // rules.extend(exactly_k(&vec![7, 4, 5, 6, 9], 1, &mut next_var));
    // // rules.extend(exactly_k(&vec![4, 5, 8], 0, &mut next_var));

    // // let mut next_var = 5; // 既存の変数が1-4を使用している場合
    // // rules.extend(at_most_k(&vec![1, 2, 3, 4], 3, &mut next_var));

    // let mut solver = Solver::try_from((config, rules.as_ref())).expect("panic");
    // let sure_model = mineguess(&mut solver, 9);
    // println!("sure_model: {:?}", sure_model);
}
