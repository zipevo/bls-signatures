use bls_signatures::G1Element;

pub fn test(bytes: &[u8]) {
    G1Element::from_bytes(&bytes).expect("should create elements from bytes");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::test(&[1, 2, 3, 4, 5, 6, 7])
    }
}
