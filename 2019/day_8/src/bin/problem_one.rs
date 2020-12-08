use std::fs;

fn main() {
    let (width, height) = (25, 6);
    let layer_size = width * height;

    let file = fs::read_to_string("input.txt").unwrap();
    
    // Ignoring bytes that don't fit into a layer, that should be just the trailing newline though.
    let slice = file.as_bytes();
    let result = slice.chunks_exact(layer_size)
        .map(|x| {
            let mut zero = 0;
            let mut one = 0;
            let mut two = 0;
            for byte in x {
                match *byte {
                    b'0' => zero += 1,
                    b'1' => one += 1,
                    b'2' => two += 1,
                    _ => continue
                }
            }
            (zero, one * two)
        })
        .min_by(|x, y| x.0.cmp(&y.0)).unwrap().1;
    
    println!("{}", result);
}
