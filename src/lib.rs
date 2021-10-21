#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

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
}
