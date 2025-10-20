use crate::command::Algorithm;
use anyhow::{Context, Result};
use data_encoding::BASE32_NOPAD;
use ring::hmac;
use ring::hmac::{HMAC_SHA1_FOR_LEGACY_USE_ONLY, HMAC_SHA256, HMAC_SHA384, HMAC_SHA512};
use std::time::SystemTime;

#[derive(Clone, Debug)]
pub struct Otp {
    key: Vec<u8>,
    totp: bool,
    counter: u64,
    length: usize,
    algorithm: Algorithm,
}

impl Otp {
    pub fn new(
        key: &str,
        algorithm: Algorithm,
        totp: bool,
        counter: Option<u64>,
        length: usize,
    ) -> Result<Self> {
        Ok(Self {
            key: BASE32_NOPAD
                .decode(key.as_bytes())
                .with_context(|| "Invalid key")?,
            length,
            counter: counter.unwrap_or(0),
            algorithm,
            totp,
        })
    }

    /// Generating an HOTP Value
    ///
    /// We can describe the operations in 3 distinct steps:
    ///
    /// Step 1: Generate an HMAC-SHA-1 value Let HS = HMAC-SHA-1(K,C)  // HS
    /// is a 20-byte string
    ///
    /// Step 2: Generate a 4-byte string (Dynamic Truncation)
    /// Let Sbits = DT(HS)   //  DT, defined below,
    /// //  returns a 31-bit string
    ///
    /// Step 3: Compute an HOTP value
    /// Let Snum  = StToNum(Sbits)   // Convert S to a number in
    /// 0...2^{31}-1
    /// Return D = Snum mod 10^Digit //  D is a number in the range
    /// 0...10^{Digit}-1
    ///
    /// The Truncate function performs Step 2 and Step 3, i.e., the dynamic
    /// truncation and then the reduction modulo 10^Digit.  The purpose of
    /// the dynamic offset truncation technique is to extract a 4-byte
    /// dynamic binary code from a 160-bit (20-byte) HMAC-SHA-1 result.
    ///
    /// DT(String) // String = String`[`0`]`...String`[`19`]`
    /// Let OffsetBits be the low-order 4 bits of String`[`19`]`
    /// Offset = StToNum(OffsetBits) // 0 <= OffSet <= 15
    /// Let P = String`[`OffSet`]`...String`[`OffSet+3`]`
    /// Return the Last 31 bits of P
    ///
    /// The following code example describes the extraction of a dynamic
    /// binary code given that hmac_result is a byte array with the HMAC-
    /// SHA-1 result:
    ///
    /// int offset   =  hmac_result`[`19`]` & 0xf ;
    /// int bin_code = (hmac_result`[`offset`]`  & 0x7f) << 24
    /// | (hmac_result`[`offset+1`]` & 0xff) << 16
    /// | (hmac_result`[`offset+2`]` & 0xff) <<  8
    /// | (hmac_result`[`offset+3`]` & 0xff) ;
    ///
    /// SHA-1 HMAC Bytes (Example)
    ///
    /// -------------------------------------------------------------
    /// | Byte Number                                               |
    /// -------------------------------------------------------------
    /// |00|01|02|03|04|05|06|07|08|09|10|11|12|13|14|15|16|17|18|19|
    /// -------------------------------------------------------------
    /// | Byte Value                                                |
    /// -------------------------------------------------------------
    /// |1f|86|98|69|0e|02|ca|16|61|85|50|ef|7f|19|da|8e|94|5b|55|5a|
    /// -------------------------------***********----------------++|
    ///
    /// * The last byte (byte 19) has the hex value 0x5a.
    /// * The value of the lower 4 bits is 0xa (the offset value).
    /// * The offset value is byte 10 (0xa).
    /// * The value of the 4 bytes starting at byte 10 is 0x50ef7f19,
    ///   which is the dynamic binary code DBC1.
    /// * The MSB of DBC1 is 0x50 so DBC2 = DBC1 = 0x50ef7f19 .
    /// * HOTP = DBC2 modulo 10^6 = 872921.
    ///
    /// We treat the dynamic binary code as a 31-bit, unsigned, big-endian
    /// integer; the first byte is masked with a 0x7f.
    ///
    /// We then take this number modulo 1,000,000 (10^6) to generate the 6-
    /// digit HOTP value 872921 decimal.
    ///
    /// We can extend above to work for totp as well as other hash algo.
    pub fn generate(&self) -> String {
        let counter = self.counter();
        let value = counter.to_be_bytes();
        let skey = hmac::Key::new(self.algorithm.hmac_algo(), self.key.as_slice());
        let tag = hmac::sign(&skey, &value);
        self.compute(tag.as_ref())
    }

    fn compute(&self, tag: &[u8]) -> String {
        let offset = (tag.last().unwrap() & 0x0f) as usize;
        let bin_code = u32::from_be_bytes([
            tag[offset] & 0x7f,
            tag[offset + 1],
            tag[offset + 2],
            tag[offset + 3],
        ]);
        let output = bin_code % (10u32.pow(self.length as u32));
        format!("{:0width$}", output, width = self.length)
    }

    fn counter(&self) -> u64 {
        if self.totp {
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                / 30
        } else {
            self.counter
        }
    }
}

impl Algorithm {
    fn hmac_algo(&self) -> hmac::Algorithm {
        match self {
            Algorithm::Sha1 => HMAC_SHA1_FOR_LEGACY_USE_ONLY,
            Algorithm::Sha256 => HMAC_SHA256,
            Algorithm::Sha384 => HMAC_SHA384,
            Algorithm::Sha512 => HMAC_SHA512,
        }
    }
}
