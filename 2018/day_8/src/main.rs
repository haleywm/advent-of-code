use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    // Reading to a vector of numbers
    let data: Vec<usize> = file
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    
    let (total, len) = rec_metadata(&data);
    assert_eq!(len, data.len());

    println!("{}", total);

    let (root_val, len) = rec_node_value(&data);
    assert_eq!(len, data.len());

    println!("{}", root_val);
}

fn rec_metadata(data: &[usize]) -> (usize, usize) {
    // Recursively reads metadata flags, returning (total, metadata tag length)
    // As there is no way of knowing what the end will be, ignores data at the end of the slice
    let chil_c = data[0];
    let meta_c = data[1];
    let mut next_start = 2;
    let mut total = 0;
    // Processing each child
    for _ in 0..chil_c {
        let result = rec_metadata(&data[next_start..]);
        total += result.0;
        next_start += result.1;
    }
    // Then processing metadata
    for i in next_start..next_start+meta_c {
        total += data[i];
    }
    next_start += meta_c;

    (total, next_start)
}

fn rec_node_value(data: &[usize]) -> (usize, usize) {
    // Meets requirements of second question
    // Returns (node value, node length)
    let chil_c = data[0];
    let meta_c = data[1];
    let mut next_start = 2;
    let mut total = 0;
    let mut children = Vec::with_capacity(chil_c);
    // Processing each child
    for _ in 0..chil_c {
        let result = rec_node_value(&data[next_start..]);
        children.push(result.0);
        next_start += result.1;
    }
    // Then processing metadata
    for i in next_start..next_start+meta_c {
        if chil_c == 0 {
            // No child nodes, total is metadata value
            total += data[i];
        }
        else if data[i] > 0 {
            // Has children, total is the value of that node if exists
            // The index must be greater than 0 as these are 1 indexed
            total += children.get(data[i] - 1).unwrap_or(&0);
        }
    }
    next_start += meta_c;

    (total, next_start)
}