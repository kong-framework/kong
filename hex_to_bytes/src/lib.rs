pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, &'static str> {
    let mut bytes = Vec::new();
    for i in 0..(hex.len() / 2) {
        let res = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16);
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => println!("Problem with hex: {}", e),
        };
    }

    Ok(bytes)
}

#[test]
fn test_hex_to_bytes() {
    let hex = "1c0111001f010100061a024b53535009181c";
    let bytes = vec![
        28, 1, 17, 0, 31, 1, 1, 0, 6, 26, 2, 75, 83, 83, 80, 9, 24, 28,
    ];
    assert_eq!(hex_to_bytes(hex).unwrap(), bytes);
}
