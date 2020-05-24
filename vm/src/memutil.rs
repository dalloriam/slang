use std::mem;

pub const WORD_WIDTH: usize = mem::size_of::<usize>();

/// Aligns the provided size value with the platform size.
///
/// # Examples
/// ```
/// use std::mem;
/// use vm::memutil::align;
///
/// let platform_word_width = mem::size_of::<usize>();
/// assert_eq!(align(3), platform_word_width);
/// ```
#[inline]
pub fn align(i: usize) -> usize {
    ((i + WORD_WIDTH - 1) / WORD_WIDTH) * WORD_WIDTH
}

#[cfg(test)]
mod tests {
    use super::align;

    #[test]
    pub fn align_0() {
        assert_eq!(align(0), 0);
    }
}
