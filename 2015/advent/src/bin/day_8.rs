fn main() {
    let values: (usize, usize) = advent::line_iter("input/day_8.txt")
        .expect("Unable to open file")
        .map(|line| {
            let line = line.unwrap();
            let bytes = line.as_bytes();
            let raw_len = bytes.len();
            let mut i = 1;
            let mut real_len = 0;
            while i < raw_len - 1 {
                if bytes[i] == b'\\' {
                    i += match bytes[i + 1] {
                        b'"' => 2,
                        b'\\' => 2,
                        b'x' => 4,
                        _ => 1
                    }
                }
                else {
                    i += 1
                }
                real_len += 1
            }
            (raw_len, real_len)
        })
        .fold((0, 0), |(cur_raw, cur_real), (add_raw, add_real)| (cur_raw + add_raw, cur_real + add_real));
    
    println!("{} - {} = {}", values.0, values.1, values.0 - values.1);

    let values: (usize, usize) = advent::line_iter("input/day_8.txt")
        .expect("Unable to open file")
        .map(|line| {
            let line = line.unwrap();
            let bytes = line.as_bytes();
            let raw_len = bytes.len();
            let mut repr_len = 2;
            for &byte in bytes {
                if byte == b'\\' || byte == b'"' {
                    repr_len += 2;
                }
                else {
                    repr_len += 1;
                }
            }
            (repr_len, raw_len)
        })
        .fold((0, 0), |(cur_raw, cur_real), (add_raw, add_real)| (cur_raw + add_raw, cur_real + add_real));
    
    println!("{} - {} = {}", values.0, values.1, values.0 - values.1);
}