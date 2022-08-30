pub mod bindings;

pub fn allocate(size: usize) -> u8 {
    unsafe {
        bindings::AllocPtrArray(size);
    }

    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(allocate(32), 1);
    }
}
