pub fn create_empty() -> Vec<u8> {
    create_buffer(0)
}

pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

pub fn fibonacci() -> Vec<u8> {
    let mut buffer = create_buffer(5);
    buffer[0] = 1;
    buffer[1] = 1;
    for i in 2..5 {
        buffer[i] = buffer[i - 1] + buffer[i - 2];
    }
    buffer
}