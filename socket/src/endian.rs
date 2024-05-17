pub fn fn_check_endian(){
    if cfg!(target_endian = "big") {
        println!("This is a BigEndian system.")
    } else {
        println!("This is a LittleEndian system.")
    }
}

pub fn twiddle_u32(data: u32) -> u32 {
    // Swap bytes using bitwise operations
    ((data & 0x000000ff) << 24) |
    ((data & 0x0000ff00) << 8) |
    ((data & 0x00ff0000) >> 8) |
    ((data & 0xff000000) >> 24)
}
pub fn twiddle_u16(data: u16) -> u16 {
    // Swap bytes using bitwise operations
    ((data & 0x00ff) << 8) |
    ((data & 0xff00) >> 8)
}