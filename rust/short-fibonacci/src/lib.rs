/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    Vec::new()
}


pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

pub fn fibonacci() -> Vec<u8> {
    let mut sequence = create_empty();
    sequence.push(1);
    sequence.push(1);
    sequence.push(2);
    sequence.push(3);
    sequence.push(5);
    sequence
}
