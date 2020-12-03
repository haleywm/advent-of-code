const TREE: char = '#';
//const EMPTY: char = '.';

pub fn line_to_bool_vec(line: &str) -> Vec<bool> {
    let mut vec = Vec::new();

    // Lazy matching, assume that non tree characters are empty.
    for letter in line.chars() {
        if letter == TREE {
            vec.push(true);
        } else {
            vec.push(false);
        }
    }

    vec
}

pub fn get_collision_count(lines: impl Iterator<Item=Vec<bool>>, direction: (i32, i32)) -> i32 {
    // Making direction i32 rather than usize so that it can be negative and will get resolved to positive by the modulus next
    let mut pos: i32 = 0;
    let mut count = 0;

    for line in lines.step_by(direction.1 as usize) {
        if line[pos as usize] {
            count += 1;
        }
        pos += direction.0;
        pos %= line.len() as i32;
    }
    
    count
}