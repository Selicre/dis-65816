pub fn to_offset(addr: u32) -> Option<usize> {
    let i = addr & 0x7FFFFF;   // ignore bit 23
    if (i < 0x400000) && (i & 0x8000 != 0) {
        let bank = i >> 16;
        let word = i & 0x7FFF;
        Some((bank*0x8000+word) as usize)
    } else {
        None
    }
}
pub fn to_address(offset: usize) -> Option<u32> {
    let inner = offset & 0x7FFF;
    let bank = offset >> 15;
    Some(((bank << 16) + inner + 0x8000) as u32)
}
