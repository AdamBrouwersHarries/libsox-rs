use raw::sox_rate_t;
use raw::sox_signalinfo_t;

type Rate = sox_rate_t;

#[derive(Debug, Copy, Clone)]
pub struct SignalInfo {
    pub rate: Rate,
    pub channels: u32,
    pub precision: u32,
    pub length: u64,
    pub mult: f64,
}

impl SignalInfo {
    pub fn from_raw(raw: sox_signalinfo_t) -> SignalInfo {
        unsafe {
            return SignalInfo {
                rate: raw.rate as Rate,
                channels: raw.channels as u32,
                precision: raw.precision as u32,
                length: raw.length as u64,
                mult: (*raw.mult) as f64,
            };
        }
    }
}
