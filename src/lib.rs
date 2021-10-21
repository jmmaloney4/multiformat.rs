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
    use num::integer::lcm;

    fn octet_group_to_ntets(input: Vec<u8>, n: u8) -> Result<Vec<u8>> {
        // N-tets can only be 1-7 in size
        ensure!(0 < n && n < 8, "Invalid N: {}", n);

        // Handle trivial case
        if input.is_empty() {
            return Ok(Vec::new());
        }

        // Number of n-tets it takes to make an even octet boundary.
        let l = lcm(8, n) / 8;
        // Verify that we are not handling more than one "group" of octets at a time.
        ensure!(
            input.len() <= l.into(),
            "Invalid input size: {} for given n: {}",
            input.len(),
            n
        );

        let mut octets = input;
        octets.resize(l.into(), 0);
        let mut output: Vec<u8> = Vec::new();

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

        Ok(output)
    }

    fn pow2(i: u8) -> u8 {
        2_u8.pow(i.into())
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {
            assert_eq!(2 + 2, 4);
            assert_eq!(
                super::octet_group_to_ntets(vec![77, 97, 110], 6).unwrap(),
                [19, 22, 5, 46]
            );
        }
    }
}
