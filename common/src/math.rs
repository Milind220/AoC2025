pub fn mod_wrap_add(pos: u32, delta: i32, modulus: i32) -> u32 {
    // little circular buffer inspired one liner
    ((((pos as i32 + delta) % modulus) + modulus) % modulus) as u32
}

pub fn count_zero_crossings(current: u32, delta: i32, modulus: i32) -> u32 {
    if delta == 0 {
        return 0;
    }
    let start = current as i32;

    let full_laps = (delta / modulus).unsigned_abs();
    let remainder = delta % modulus;
    let unwrapped_end = start + remainder;

    if start != 0 {
        // Now we need to check if the remainder causes us to cross zero.
        // We are effectively checking to see if there was overflow in either direction.
        let crossed_in_remainder = (unwrapped_end <= 0) || (unwrapped_end >= modulus);
        return full_laps + crossed_in_remainder as u32;
    }
    full_laps
}
