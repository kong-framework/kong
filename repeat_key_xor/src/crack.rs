use hamming_distance::calc as hamm_dist;
use hex_to_bytes::hex_to_bytes;

#[derive(Debug, Clone)]
struct Candidate {
    keysize: usize,
    distance: usize,
}

#[derive(Debug)]
struct MostLikely {
    values: Vec<usize>,
}

struct Crack<'a> {
    cyphertext: &'a [u8],
    max_keysize: usize,
    most_likely: Option<Candidate>,
}

impl<'a> Crack<'a> {
    pub fn new(cyphertext: &'a [u8], max_keysize: usize) -> Self {
        Crack {
            cyphertext,
            max_keysize,
            most_likely: None,
        }
    }

    pub fn get_most_likely(&mut self) -> Result<(), ()> {
        let mut candidates: Vec<Candidate> = vec![];

        for keysize in 2..self.max_keysize {
            let ks1 = &self.cyphertext[0..keysize];
            let ks2 = &self.cyphertext[keysize..keysize * 2];

            if let Some(distance) = hamm_dist(ks1, ks2) {
                let ks_1 = &self.cyphertext[0..keysize];
                let ks_2 = &self.cyphertext[keysize..keysize * 2];
                let ks_3 = &self.cyphertext[keysize * 2..keysize * 3];
                let ks_4 = &self.cyphertext[keysize * 3..keysize * 4];

		for 0..12 {

		}

                candidates.push(Candidate {
                    distance: distance as usize / keysize,
                    keysize,
                })
            };
        }

        candidates.sort_by(|a, b| a.distance.cmp(&b.distance));

        self.most_likely = Some(candidates[0].clone());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_finder() {
        let cyphertext = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

        let cyphertext_bytes = hex_to_bytes(cyphertext).unwrap();
        let mut crack = Crack::new(&cyphertext_bytes, 35);

        crack.get_most_likely().unwrap();
        crack.get_most_likely().unwrap();

        if let Some(candidate) = crack.most_likely {
            assert_eq!(candidate.keysize, 3);
        }
    }
}
