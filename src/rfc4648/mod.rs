mod rfc4648 {

    use anyhow::{ensure, Result};
    use num::integer::{div_ceil, div_floor, lcm};

    #[derive(Debug)]
    enum Alphabet {
        Binary,
        Octal,
        Base16,
        Base16Upper,
        Base32,
        Base32Hex,
        Base32Upper,
        Base32HexUpper,
        Base64,
        Base64Url,
    }

    impl Alphabet {
        const PADDING_CHAR: char = '=';

        fn alphabet(&self) -> Vec<char> {
            match self {
                Alphabet::Binary => "01",
                Alphabet::Octal => "01234567",
                Alphabet::Base16 => "0123456789abcdef",
                Alphabet::Base16Upper => "0123456789ABCDEF",
                Alphabet::Base32 => "abcdefghijklmnopqrstuvwxyz234567",
                Alphabet::Base32Hex => "0123456789abcdefghijklmnopqrstuv",
                Alphabet::Base32Upper => "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567",
                Alphabet::Base32HexUpper => "0123456789ABCDEFGHIJKLMNOPQRSTUV",
                Alphabet::Base64 => {
                    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
                }
                Alphabet::Base64Url => {
                    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_"
                }
            }
            .chars()
            .collect()
        }

        fn len(&self) -> usize {
            self.alphabet().len()
        }

        fn bits_per_char(&self) -> f64 {
            (self.len() as f64).log2()
        }
    }

    trait AlphabetCodable {
        fn to_alphabet_str(&self, alphabet: Alphabet) -> String;
        fn from_alphabet_str(s: String, alphabet: Alphabet) -> Self;
    }

    impl AlphabetCodable for Vec<u8> {
        fn to_alphabet_str(&self, alphabet: Alphabet) -> String {
            self.iter()
                .map(|k| -> char {
                    assert!(
                        *k < alphabet.len() as u8,
                        "No corresponding alphabet character for {}",
                        k
                    );
                    alphabet.alphabet()[*k as usize]
                })
                .collect::<String>()
        }

        fn from_alphabet_str(s: String, alphabet: Alphabet) -> Self {
            s.chars()
                .filter(|c| -> bool { *c != Alphabet::PADDING_CHAR })
                .map(|c| -> u8 {
                    match alphabet.alphabet().iter().position(|a| -> bool { *a == c }) {
                        Some(rv) => rv as u8,
                        None => {
                            panic!("{} not present in alphabet {:?}", c, alphabet);
                        }
                    }
                })
                .collect()
        }
    }

    /*
     *  Base 64:
     *
     *      +--first octet--+-second octet--+--third octet--+
     *      |7 6 5 4 3 2 1 0|7 6 5 4 3 2 1 0|7 6 5 4 3 2 1 0|
     *      +-----------+---+-------+-------+---+-----------+
     *      |5 4 3 2 1 0|5 4 3 2 1 0|5 4 3 2 1 0|5 4 3 2 1 0|
     *      +--1.index--+--2.index--+--3.index--+--4.index--+
     *
     *  Base 32:
     *
     *      01234567 89012345 67890123 45678901 23456789
     *      +--------+--------+--------+--------+--------+
     *      |< 1 >< 2| >< 3 ><|.4 >< 5.|>< 6 ><.|7 >< 8 >|
     *      +--------+--------+--------+--------+--------+
     *
     *  Base 2:
     *      012345678
     *
     */

    fn octet_group_to_ntets(input: &Vec<u8>, n: u8) -> Result<Vec<u8>> {
        // N-tets can only be 1-7 in size
        ensure!(0 < n && n < 8, "Invalid N: {}", n);

        // Handle trivial case
        if input.is_empty() {
            return Ok(Vec::new());
        }

        // Octet group size
        let ogs: u8 = lcm(8, n) / 8;
        // N-tet group size
        let ngs: u8 = lcm(8, n) / n;

        // Number of *full* octet groups in the input
        let full_groups: usize = div_floor(input.len(), ogs as usize);
        // Number of octets padded out to a full group.
        let total_groups: usize = div_ceil(input.len(), ogs as usize);

        // Number of octets in input which are not in a full group.
        let residual_octets: u8 = input.len() as u8 % ogs;
        // Number of n-tets needed to fully cover residual octets.
        let residual_ntets: u8 = div_ceil(residual_octets * 8, n);

        // Number of ntets we will output.
        let out_len: usize = (full_groups * ngs as usize) + residual_ntets as usize;

        // Vector to store the output
        let mut output: Vec<u8> = Vec::with_capacity(total_groups * ngs as usize);

        // Pad input out to a full octet group.
        let input_pad = vec![&0_u8; total_groups * ogs as usize - input.len()];
        let mut iter = input.iter().chain(input_pad.into_iter());

        // Offset between the start of an octet and the start of the n-tet
        let mut offset: u8 = n;

        // We already checked that the input is not empty.
        let mut octet: u8 = *iter.next().unwrap();
        // Residual from the last octet that belongs in the next n-tet
        let mut carry: u8 = 0;

        loop {
            let q = octet / pow2((8 - offset) % 8);
            let r = octet % pow2((8 - offset) % 8);

            output.push(carry * pow2(offset) + q);
            offset += n;

            if offset < 8 {
                // Have not crossed an octet boundary, keep parsing this octet.
                octet = r;
                carry = 0;
            } else {
                // Have reached or crossed an octet boundary. Try to fetch the next octet.
                match iter.next() {
                    Some(next) => {
                        // There is another octet. Preserve
                        // the remainder (the lower half) of the current octet, because
                        // it will be the upper segment of the next n-tet.
                        octet = *next;
                        if offset == 8 {
                            // Reached the end of a group, exactly at an octet boundary.
                            // The remainder carries an entire n-tet, so just put in in the output.
                            output.push(r);
                            carry = 0;
                            // Bump the offset over, because we have handeled this last n-tet.
                            offset += n;
                        } else {
                            // Otherwise the remainder is the upper part of the next n-tet, so we
                            // need to roll it over.
                            carry = r;
                        }
                    }
                    None => {
                        // There are no more octets. The last n-tet is just the remainder
                        // of the current octet, but shifted up to the left most significant
                        // side of the n-tet.
                        output.push(r * pow2(offset % 8));
                        break;
                    }
                }
            }

            offset %= 8;
        }

        output.truncate(out_len);
        Ok(output)
    }

    fn ntet_group_to_octets(input: &Vec<u8>, n: u8) -> Result<Vec<u8>> {
        // N-tets can only be 1-7 in size
        ensure!(0 < n && n < 8, "Invalid N: {}", n);

        // Handle trivial case
        if input.is_empty() {
            return Ok(Vec::new());
        }

        // Octet group size
        let ogs: u8 = lcm(8, n) / 8;
        // N-tet group size
        let ngs: u8 = lcm(8, n) / n;

        // Number of *full* n-tet groups in the input
        let full_groups: usize = div_floor(input.len(), ngs as usize);
        // Number of n-tets padded out to a full group.
        let total_groups: usize = div_ceil(input.len(), ngs as usize);

        // Number of n-tets in input which are not in a full group.
        let residual_ntets: u8 = input.len() as u8 % ngs;
        // Number of octets fully covered by residual n-tets.
        let residual_octets: u8 = div_floor(residual_ntets * n, 8);

        // Number of octets we will output.
        let out_len: usize = (full_groups * ogs as usize) + residual_octets as usize;

        let mut output = vec![0; total_groups * ogs as usize];
        // Index into output
        let mut j: usize = 0;

        let mut offset: u8 = 0;
        let mut q: u8;
        let mut r: u8 = 0;

        let input_pad = vec![&0_u8; total_groups * ngs as usize - input.len()];
        for i in input.iter().chain(input_pad.into_iter()) {
            ensure!(*i < pow2(n), "Invalid {}-tet: {}", n, i);

            // Handle carry. Take the least significant part of the n-tet (r) and
            // move it to the most significant part of the next output byte.
            if r != 0 {
                output[j] += r * pow2(8 - offset);
            }
            // We have handled the remainder now.
            r = 0;

            offset += n;
            if offset < 8 {
                // We have not crossed a byte boundary, so the entire input n-tet is in the current output byte.
                output[j] += i * pow2(8 - offset);
            } else {
                // We have crossed a byte boundary, we need to split the most and least significant halves of the input n-tet
                q = i / pow2(offset - 8);
                r = i % pow2(offset - 8);
                output[j] += q;
                j += 1;
            }

            offset = offset % 8
        }

        ensure!(
            r == 0
                && output
                    .split_off(out_len)
                    .iter()
                    .all(|i| -> bool { *i == 0 as u8 }),
            "Non-canonical N-tet"
        );

        // Already appropriately truncated by .split_off above
        Ok(output)
    }

    fn pow2(i: u8) -> u8 {
        assert!(i < 8);
        2_u8.pow(i.into())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::array::IntoIter;
        use std::collections::HashMap;
        use std::iter::FromIterator;

        #[test]
        fn test_octet_ntet_conversion() {
            let cases_6 = HashMap::<Vec<u8>, Vec<u8>>::from_iter(IntoIter::new([
                (vec![], vec![]),
                (vec![77, 97, 110], vec![19, 22, 5, 46]),
                (vec![77, 97], vec![19, 22, 4]),
                (vec![77], vec![19, 16]),
                (
                    vec![102, 111, 111, 98, 97, 114],
                    vec![25, 38, 61, 47, 24, 38, 5, 50],
                ),
            ]));

            let cases_5 = HashMap::<Vec<u8>, Vec<u8>>::from_iter(IntoIter::new([(
                vec![102, 111, 111],
                vec![12, 25, 23, 22, 30],
            )]));

            let cases_4 =
                HashMap::<Vec<u8>, Vec<u8>>::from_iter(IntoIter::new([(vec![102], vec![6, 6])]));

            let cases_3 =
                HashMap::<Vec<u8>, Vec<u8>>::from_iter(IntoIter::new([(vec![23], vec![0, 5, 6])]));

            let cases_1 = HashMap::<Vec<u8>, Vec<u8>>::from_iter(IntoIter::new([(
                vec![201, 111, 111],
                vec![
                    1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1,
                ],
            )]));

            let test_n = |cases: HashMap<Vec<u8>, Vec<u8>>, n: u8| {
                for (i, o) in cases {
                    assert_eq!(octet_group_to_ntets(&i, n).unwrap(), o);
                    assert_eq!(ntet_group_to_octets(&o, n).unwrap(), i);
                }
            };

            test_n(cases_6, 6);
            test_n(cases_5, 5);
            test_n(cases_4, 4);
            test_n(cases_3, 3);
            test_n(cases_1, 1);
        }
    }
}
