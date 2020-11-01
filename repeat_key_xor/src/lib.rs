pub mod crack;

/// Encrypts `cleartext` using a repeated key
/// [XOR cipher.](https://en.wikipedia.org/wiki/XOR_cipher)
pub fn encrypt(cleartext: &[u8], key: &[u8]) -> std::io::Result<String> {
    let mut key_position = 0;
    let mut cyphertext: Vec<String> = vec![];

    for c in cleartext {
        if key_position == key.len() {
            key_position = 0;
        };

        cyphertext.push(format!("{:02x}", c ^ key[key_position]));

        key_position += 1;
    }

    Ok(cyphertext.join(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let cleartext = b"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
        let key = b"ICE";

        let cyphertext = String::from("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");

        assert_eq!(encrypt(cleartext, key).unwrap(), cyphertext);
    }
}
