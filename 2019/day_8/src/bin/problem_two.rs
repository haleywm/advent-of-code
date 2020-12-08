use std::fs;

fn main() {
    let (width, height) = (25, 6);
    let layer_size = width * height;

    let file = fs::read_to_string("input.txt").unwrap();
    
    // Ignoring bytes that don't fit into a layer, that should be just the trailing newline though.
    let slice = file.as_bytes();
    let result = String::from_utf8(
            slice.chunks_exact(layer_size)
            .fold(vec![b'2'; layer_size], |mut image, layer| {
                for (i, val) in layer.iter().enumerate() {
                    if image[i] == b'2' {
                        // Only modifying layers that are currently transparent
                        image[i] = match *val {
                            b'0' => b'#',
                            b'1' => b' ',
                            x => x,
                        };
                    }
                }
                image
            })
    ).unwrap();
    
    for y in 0..height {
        println!("{}", &result[y * width..(y + 1) * width]);
    }
}
