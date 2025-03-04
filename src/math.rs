#[inline(always)]
pub fn ror32(value: u32, count: u32) -> u32 {
    value.rotate_right(count)
}

