use std::fs;
use std::io::{self, BufRead};
use std::fmt;
use core::convert::TryFrom;

const HEALTH: i32 = 200;
const GOBLIN_ATTACK: i32 = 3;

type Grid = Vec<Vec<Tile>>;

#[derive(Eq, PartialEq, Clone, Debug)]
enum Tile {
    Empty,
    Wall,
    Unit(bool, Unit),
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Unit(unit_type, _) => {
                if *unit_type {
                    'E'
                }
                else {
                    'G'
                }
            },
        })
    }
}


impl TryFrom<u8> for Tile {
    type Error = &'static str;

    fn try_from(input: u8) -> Result<Self, Self::Error> {
        match input {
            b'.' => Ok(Tile::Empty),
            b'#' => Ok(Tile::Wall),
            b'E' => Ok(Tile::Unit(true, Unit::default())),
            b'G' => Ok(Tile::Unit(false, Unit::default())),
            _ => Err("Unrecognized input value"),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Unit {
    health: i32,
    last_turn: usize,
}

impl Default for Unit {
    fn default() -> Self { Self { health: HEALTH, last_turn: 0 } }
}

fn read_input() -> Result<Grid, &'static str> {
    const BAD_FILE: &str = "Invalid input file";
    let file = fs::File::open("input.txt").or(Err("Unable to read file"))?;
    let lines = io::BufReader::new(file)
        .lines();
    
    let mut grid = Vec::new();
    let mut line_len = None;

    for line in lines {
        let line = line.unwrap();
        let len = line.len();
        match line_len {
            Some(val) => if len != val {
                return Err(BAD_FILE);
            }
            None => line_len = Some(len),
        }
        let mut grid_line = Vec::with_capacity(len);
        for in_char in line.as_bytes() {
            grid_line.push(Tile::try_from(*in_char)?);
        }

        grid.push(grid_line);
    }

    if grid.len() == 0 || grid[0].len() == 0 {
        Err(BAD_FILE)
    }
    else {
        Ok(grid)
    }
}

fn main() {
    match read_input() {
        Ok(grid) => {
            // Part 1
            let (winner, turn, health) = run_battle(grid.clone(), GOBLIN_ATTACK, false);
            let winner = if winner { "Elves" } else { "Goblins" };
            println!("Part 1 Outcome: {} win, {} * {} = {}", winner, turn, health, turn * health);

            // Part 2
            let mut elf_power = GOBLIN_ATTACK + 1;
            let (turn, health) = loop {
                let (winner, turn, health) = run_battle(grid.clone(), elf_power, true);
                if winner {
                    // Done!
                    break (turn, health);
                }
                elf_power += 1;
            };
            println!("Part 2 Outcome: Elf Power {}, {} * {} = {}", elf_power, turn, health, turn * health);
        }
        Err(msg) => {
            eprintln!("{}", msg);
        }
    }
}

fn run_battle(mut grid: Grid, elf_power: i32, no_elf_deaths: bool) -> (bool, usize, usize) {
    // Runs a battle, returning the winning team, the number of turns, and the amount of health
    // Perfoming turns
    let mut turn = 1;
    loop {
        //println!("Turn {}:", turn);
        //print_grid(&grid);
        // Looping
        let max_y = grid.len() - 1;
        let max_x = grid[0].len() - 1;
        for y in 0..=max_y{
            for x in 0..=max_x {
                if let Tile::Unit(unit_type, unit) = &grid[y][x] {
                    if unit.last_turn < turn {
                        let unit_type = unit_type.clone();
                        let mut final_x = x;
                        let mut final_y = y;
                            
                        if let None = get_adjacent_enemy(&grid, !unit_type, y, x) {
                            // Nothing adjacent to attack, instead finding a place to move
                            let (count, adjacent) = get_adjacent_locations(&grid, !unit_type);
                            if count == 0 {
                                print_grid(&grid);
                                let turn = turn - 1;
                                let health: i32 = grid
                                    .into_iter()
                                    .flatten()
                                    .filter_map(|x| {
                                        match x {
                                            Tile::Unit(_, unit) => Some(unit.health),
                                            _ => None,
                                        }
                                    })
                                    .sum();
                                let health = health as usize;
                                return (unit_type, turn, health);
                            }
                            let to = generate_next_point(&grid, &adjacent, (y, x));
                            if let Some((to_y, to_x)) = to {
                                assert_eq!(grid[to_y][to_x], Tile::Empty);
                                grid[to_y][to_x] = grid[y][x].clone();
                                grid[y][x] = Tile::Empty;
                                final_y = to_y;
                                final_x = to_x;
                            }
                        }
                        // After the first attempt to move, then seeing if an attack can be made again and performing if possible
                        if let Some((enemy_y, enemy_x)) = get_adjacent_enemy(&grid, !unit_type, final_y, final_x) {
                            // Attacking
                            if let Tile::Unit(_, enemy) = &mut grid[enemy_y][enemy_x] {
                                enemy.health -= if unit_type {
                                    elf_power
                                }
                                else {
                                    GOBLIN_ATTACK
                                };
                                if enemy.health <= 0 {
                                    if !unit_type && no_elf_deaths {
                                        return (false, 0, 0);
                                    }
                                    grid[enemy_y][enemy_x] = Tile::Empty;
                                }
                            }
                            else {
                                assert!(false);
                            }
                        }
                        if let Tile::Unit(_, unit) = &mut grid[final_y][final_x] {
                            unit.last_turn = turn;
                        }
                    }
                }
                // Now that earlier references have been closed, updating last tick if needed
            }
        }
        turn += 1;
    }
}

fn get_adjacent_enemy(grid: &Grid, to_find: bool, cur_y: usize, cur_x: usize) -> Option<(usize, usize)> {
    let mut least_points = (201, 0, 0);
    for dir in 0..4 {
        let (y, x) = adjust_pos((cur_y, cur_x), dir);
        if let Tile::Unit(unit_type, unit) = &grid[y][x] {
            if *unit_type == to_find {
                if unit.health < least_points.0 {
                    least_points = (unit.health, y, x);
                }
            }
        }
    }
    if least_points.1 != 0 && least_points.2 != 0 {
        Some((least_points.1, least_points.2))
    }
    else {
        None
    }
}

fn get_adjacent_locations(grid: &Grid, to_find: bool) -> (usize, Vec<(usize, usize)>) {
    // Returns the number of enemies, as well as their free adjacent tiles
    let max_y = grid.len() - 1;
    let max_x = grid[0].len() - 1;

    let mut positions = Vec::new();
    let mut count = 0;

    for y in 0..=max_y {
        for x in 0..=max_x {
            if let Tile::Unit(unit_type, _) = grid[y][x] {
                if unit_type == to_find {
                    // Found an enemy position. Checking adjacent positions to see if they are free
                    count += 1;
                    for dir in 0..4 {
                        let (check_y, check_x) = adjust_pos((y, x), dir);
                        if let Tile::Empty = grid[check_y][check_x] {
                            // Found an empty pos!
                            positions.push((check_y, check_x));
                        }
                    }
                }
            }
        }
    }

    (count, positions)
}

fn generate_next_point(from_grid: &Grid, points: &Vec<(usize, usize)>, target: (usize, usize)) -> Option<(usize, usize)> {
    // Takes a grid, and a list of points on that grid, and outputs a new grid of equal size with distance to given points
    // Stops when it finds points adjacent to target
    let mut to_grid = vec![vec![false; from_grid[0].len()]; from_grid.len()];
    for &(y, x) in points {
        to_grid[y][x] = true;
    }

    loop {
        // Checking if there's something adjacent to go to, (in to-visit order to make sure path is followed correctly)
        for dir in 0..4 {
            let (comp_y, comp_x) = adjust_pos(target, dir);
            if to_grid[comp_y][comp_x] {
                // Found an adjacent pos!
                return Some((comp_y, comp_x));
            }
        }
        let mut new_to_grid = to_grid.clone();
        let mut progress_made = false;
        for y in 1..to_grid.len() - 1 {
            for x in 1..to_grid[0].len() - 1 {
                if !to_grid[y][x] {
                    if let Tile::Empty = from_grid[y][x] {
                        for dir in 0..4 {
                            let (comp_y, comp_x) = adjust_pos((y, x), dir);
                            if to_grid[comp_y][comp_x] {
                                // Found an adjacent pos!
                                new_to_grid[y][x] = true;
                                progress_made = true;
                            }
                        }
                    }
                }
            }
        }
        to_grid = new_to_grid;
        if !progress_made {
            // There is no path
            return None;
        }
    }
}

fn adjust_pos(pos: (usize, usize), dir: usize) -> (usize, usize) {
    match dir {
        0 => (pos.0 - 1, pos.1),
        1 => (pos.0, pos.1 - 1),
        2 => (pos.0, pos.1 + 1),
        3 => (pos.0 + 1, pos.1),
        _ => panic!("No"),
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    for line in grid.iter() {
        let mut after = String::new();
        for tile in line.iter() {
            if let Tile::Unit(_, unit) = tile {
                after.push_str(&format!(" {}({}),", tile, unit.health));
            }
            print!("{}", tile);
        }
        println!("{}", after);
    }
}