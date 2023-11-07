pub fn reverse_endianness(word: u16) -> u16 {
    let byte_mask: u16 = 0xFF;
    let new_high_byte: u16 = byte_mask & word;
    let new_low_byte: u16 = ((byte_mask << 8) & word) >> 8;
    let new_word: u16 = (new_high_byte << 8) | new_low_byte;
    return new_word;
}
