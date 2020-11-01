/// Computes the Hamming distance of two equal length byte arrays
pub fn hamming_distance(a: &[u8], b: &[u8]) -> Option<u32> {
    if a.len() != b.len() {
        return None;
    }

    let count = a
        .iter()
        .zip(b.iter())
        .map(|(i, j)| (i ^ j).count_ones())
        .sum();

    Some(count)
}

#[test]
fn test_hamming_distance() {
    assert_eq!(hamming_distance(b"k", b"kathrin"), None);
    assert_eq!(
        hamming_distance(b"this is a test", b"wokka wokka!!!"),
        Some(37)
    );
}
