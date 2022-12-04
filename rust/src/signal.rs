use std::collections::HashSet;

pub fn start_of_packet(string : &String, len : usize) -> usize {
    for (i , bs) in string.as_bytes().windows(len).enumerate() {
        let set : HashSet<u8> = HashSet::from_iter(bs.iter().cloned());
        if set.len() == len {
            return i + len;
        }
    }
    0
} 