pub struct Letter<'a> {
    pub character: &'a str,
    pub count: u32
}

pub struct LetterCount<'a> {
    pub letters: Vec<Letter<'a>>
}

pub fn count_letters(cyphertext: &str) -> Result<LetterCount, ()> {
    let mut letter_count: Vec<Letter>  = vec![];

    for c in cyphertext.split("") {
        let temp = letter_count.iter().position(|x| x.character == c);

        if let Some(i) = temp {
            letter_count[i].count = letter_count[i].count + 1;
        } else {
            letter_count.push(Letter{character: c, count: 1});
        }
    }

    Ok(LetterCount{letters: letter_count})
}

pub fn chi_sqr(key: &str) -> f32{
    let expected = vec![
        (b"A", 0.08167),
        (b"B", 0.01492),
        (b"C", 0.02782),
        (b"D", 0.04253),
        (b"E", 0.12702),
        (b"F", 0.02228),
        (b"G", 0.02015),
        (b"H", 0.06094),
        (b"I", 0.06966),
        (b"J", 0.00153),
        (b"K", 0.00772),
        (b"L", 0.04025),
        (b"M", 0.02406),
        (b"N", 0.06749),
        (b"O", 0.07507),
        (b"P", 0.01929),
        (b"Q", 0.00095),
        (b"R", 0.05987),
        (b"S", 0.06327),
        (b"T", 0.09056),
        (b"U", 0.02758),
        (b"V", 0.00978),
        (b"W", 0.02360),
        (b"X", 0.00150),
        (b"Y", 0.01974),
        (b"Z", 0.00074)
    ];


    let analysis = count_letters(key).unwrap();
    let mut sum: f32 = 0.0;

    for letter in analysis.letters {
        let index = expected.iter().position(
            |&x| x.0 == letter.character.to_ascii_uppercase().as_bytes()
        );
        
        if let Some(i) = index {
            let expected_count = expected[i].1 * (key.len() as f32);
            let temp = (letter.count as f32 - expected_count).powi(2) / expected_count;
            sum = sum + temp;
        } 
    }

    sum
}
