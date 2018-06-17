use raw::sox_bool;
use raw::sox_encoding_t;
use raw::sox_encodinginfo_t;
use raw::sox_option_t;

// TODO: Make an enum
type Encoding = sox_encoding_t;

// TODO: Make an enum
type Option = sox_option_t;

// TODO: Make an enum
type SBool = sox_bool;

#[derive(Debug, Copy, Clone)]
pub struct EncodingInfo {
    pub encoding: Encoding,
    pub bits_per_sample: u32,
    pub compression: f64,
    pub reverse_bytes: Option,
    pub reverse_nibbles: Option,
    pub reverse_bits: Option,
    pub opposite_endian: SBool,
}

impl EncodingInfo {
    pub fn from_raw(raw: sox_encodinginfo_t) -> EncodingInfo {
        unsafe {
            return EncodingInfo {
                encoding: raw.encoding,
                bits_per_sample: raw.bits_per_sample,
                compression: raw.compression,
                reverse_bytes: raw.reverse_bytes,
                reverse_nibbles: raw.reverse_nibbles,
                reverse_bits: raw.reverse_bits,
                opposite_endian: raw.opposite_endian,
            };
        }
    }
}
