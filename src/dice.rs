use rand;

pub fn d6() -> u8 {
    return rand::random::<u8>() % 5 + 1
}
