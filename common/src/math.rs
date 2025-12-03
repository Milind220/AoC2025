pub fn mod_wrap_add(pos: u32, delta: i32, mod_total: i32) -> u32 {
    // little circular buffer inspired one liner
    ((((pos as i32 + delta) % mod_total) + mod_total) % mod_total) as u32
}
