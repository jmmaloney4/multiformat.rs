pub mod multibase {
    #[repr(u8)]
    pub enum Encoding {
        /// "\0"
        Identity = b'\0',
        /// "0"
        Base2 = b'0',
        /// "7"
        Base8 = b'7',
        /// "9"
        Base10 = b'9',
        /// "f"
        Base16 = b'f',
        /// "F"
        Base16Upper = b'F',
        /// "v"
        Base32Hex = b'v',
        /// "V"
        Base32HexUpper = b'V',
        /// "t"
        Base32HexPad = b't',
        /// "T"
        Base32HexPadUpper = b'T',
        /// "b"
        Base32 = b'b',
        /// "B"
        Base32Upper = b'B',
        /// "c"
        Base32Pad = b'c',
        /// "C"
        Base32PadUpper = b'C',
        /// "h"
        Base32z = b'h',
        /// "k"
        Base36 = b'k',
        /// "K"
        Base36Upper = b'K',
        /// "z"
        Base58BTC = b'z',
        /// "Z"
        Base58Flickr = b'Z',
        /// "m"
        Base64 = b'm',
        /// "M"
        Base64Pad = b'M',
        /// "u"
        Base64Url = b'u',
        /// "U"
        Base64UrlPad = b'U',
        /// "p"
        Proquint = b'p',
    }

    pub struct Multibase {}

    use anyhow::{ensure, Result};
    use num::integer::{div_ceil, div_floor, lcm};

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

    fn octet_group_to_ntets(input: Vec<u8>, n: u8) -> Result<Vec<u8>> {
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

        // Pad input out to a full octet group.
        let mut octets = input;
        octets.resize(total_groups * ogs as usize, 0);
        let mut output: Vec<u8> = Vec::with_capacity(total_groups * ngs as usize);

        // Offset between the start of an octet and the start of the n-tet
        let mut offset: u8 = n;
        // Which octet we are currently translating
        let mut octets_i: usize = 0;

        // Initialize variables used during loop
        let mut octet: u8 = octets[octets_i];
        // Residual from the last octet that belongs in the next n-tet
        let mut carry: u8 = 0;

        loop {
            let q = octet / pow2(8 - offset);
            let r = octet % pow2(8 - offset);
            println!("{} {} {} {} {}", offset, carry, octet, q, r);

            output.push(carry * pow2(offset) + q);
            offset += n;
            if offset < 8 {
                octet = r;
                carry = 0;
            } else {
                octets_i += 1;
                if octets_i < octets.len() {
                    carry = r;
                    octet = octets[octets_i];
                } else {
                    output.push(r);
                    break;
                }
            }

            offset %= 8
        }

        output.truncate(out_len.into());
        Ok(output)
    }
    /*
    fn ntet_group_to_octets(input: Vec<u8>, n: u8) -> Result<Vec<u8>> {
        // N-tets can only be 1-7 in size
        ensure!(0 < n && n < 8, "Invalid N: {}", n);

        // Handle trivial case
        if input.is_empty() {
            return Ok(Vec::new());
        }

        // Octet group size
        let ogs = lcm(8, n) / 8;
        // N-tet group size
        let ngs = lcm(8, n) / n;

        let out_len = div_floor(input.len() * n as usize, 8);

        let mut ntets = input;
        ntets.resize(m.into(), 0);

        let mut output = vec![0; (lcm(8, n) / 8).into()];
        // Index into output
        let mut j: usize = 0;

        let mut offset: u8 = 0;
        let mut q: u8;
        let mut r: u8 = 0;

        for i in ntets.into_iter() {
            ensure!(i < pow2(n), "Invalid {}-tet: {}", n, i);

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

        let x = output.split_off(out_len);
        println!("{:#?}", x);

        ensure!(
            r == 0 && x.iter().all(|i| -> bool { *i == 0 as u8 }),
            "Not Canonical N-Tet"
        );

        // Already appropriately truncated by .split_off above
        Ok(output)
    }
    */

    fn pow2(i: u8) -> u8 {
        2_u8.pow((i % 8).into())
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn test_octet_group_to_ntets() {
            // assert_eq!(
            //     super::octet_group_to_ntets(vec![77, 97, 110], 6).unwrap(),
            //     [19, 22, 5, 46]
            // );

            // assert_eq!(
            //     super::octet_group_to_ntets(vec![77, 97], 6).unwrap(),
            //     [19, 22, 4]
            // );

            // assert_eq!(super::octet_group_to_ntets(vec![77], 6).unwrap(), [19, 16]);

            assert_eq!(
                super::octet_group_to_ntets(vec![102, 111, 111, 98, 97, 114], 6).unwrap(),
                // Zm9vYmFy 
                [25, 38, 61, 47, 24, 38, 5, 50]
            );
        }
        /*
        #[test]
        fn test_ntet_group_to_octets() {
            assert_eq!(
                super::ntet_group_to_octets(vec![19, 22, 5, 46], 6).unwrap(),
                [77, 97, 110]
            );

            assert_eq!(
                super::ntet_group_to_octets(vec![19, 22, 4], 6).unwrap(),
                [77, 97]
            );

            assert_eq!(super::ntet_group_to_octets(vec![19, 16], 6).unwrap(), [77]);
        }
        */
    }
}
