const REM: u64 = 20201227;
const DEF_SUB: u64 = 7;

pub fn forward_enc(subject: u64, loop_size: usize) -> u64 {
    // Performs the transformation forward as specified
    let mut val = 1;
    for _ in 0..loop_size {
        val *= subject;
        val %= REM;
    }
    
    val
}

pub fn crack_loop(key: u64) -> usize {
    // Calculate the loop size from the final key, by knowing the algorithm used
    // As well as the default subject

    // Easiest solution: Continue looping from the default value until key is met
    let mut i = 0;
    let mut val = 1;
    loop {
        if val == key {
            // Found it!
            return i;
        }
        
        i += 1;
        val *= DEF_SUB;
        val %= REM;
    }
}