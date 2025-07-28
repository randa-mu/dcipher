/// Safe u32 to usize conversion on 32 bits and 64 bits platform
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
#[inline(always)]
pub(crate) const fn u32_to_usize(x: u32) -> usize {
    x as usize
}

/// Safe usize to u64 conversion on 32 bits and 64 bits platform
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
#[inline(always)]
#[allow(unused)]
pub(crate) const fn usize_to_u64(x: usize) -> u64 {
    x as u64
}
