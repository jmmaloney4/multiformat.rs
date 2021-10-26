
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

pub enum DecodeError {

}

pub enum EncodeError {

}

pub enum OtherError {

}

/// Options for multibase encoding.
pub struct EncodingOptions { 
    /// The encoding to use.
    encoding: Encoding,
    /// Prefix the encoded string with the multibase letter.
    prefix: bool,
    /// For encodings which support it, pad the output with `padding_char`.
    pad: bool,
    /// The character to pad the output with if `pad` is true.
    padding_char: char,
}

impl EncodingOptions {
    fn default(encoding: Encoding) -> Self {
        EncodingOptions::new(encoding, None, None, None);
    }

    fn new(encoding: Encoding, prefix: Option<bool>, pad: Option<bool>, padding_char: Option<char>) -> Self {
        EncodingOptions {
            encoding: encoding,
            prefix: match prefix {
                Some(prefix) => prefix,
                None => true
            },
            pad: match pad {
                Some(pad) => pad,
                None => true
            },
            padding_char: match padding_char {
                Some(padding_char) => padding_char,
                None => '='
            }
        }
    }
}

pub struct DecodingOptions {
    /// Force decode with given encoding. If `None`, detect encoding using multibase prefix.
    encoding: Option<Encoding>
    
}


pub fn detect_encoding(string: String) -> Encoding {
    Encoding::Base64
}

pub fn encode<T: AsRef<[u8]>>(input: T, encoding: Encoding) -> String {
    encode_options(input, EncodingOptions::default(encoding))
}

pub fn encode_options<T: AsRef<[u8]>>(input: T, options: EncodingOptions) -> String {

}

pub fn decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, DecodeError> {
    decode_options(input, encoding: Encoding)
}

pub fn decode_options<T: AsRef<[u8]>>(input: T, options: Encoding) -> Result<Vec<u8>, DecodeError> {

}