use core::mem;

/// Stateful vli decoder.
#[derive(Clone, Copy, Default, Debug)]
pub struct VliDecoder {
    /// The vli as far as it has already been decoded.
    vli: u64,
    /// How many bits we already decoded, values about 62 are invalid.
    vli_bits: u8,
}

impl VliDecoder {
    /// Constructor
    pub const fn new() -> Self {
        Self {
            vli: 0,
            vli_bits: 0,
        }
    }

    /// Reset back to initial state
    pub const fn reset(&mut self) {
        self.vli_bits = 0;
        self.vli = 0;
    }

    /// Decodes a vli with a buffer that is known to hold the full vli.
    /// Returns (decoded vli, amount of bytes consumed)
    ///
    /// Returns None on `InvalidVli` or Insufficient data. This is fatal
    /// and the decoder will not produce correct output anymore until reset.
    pub fn decode_single(&mut self, input: &[u8]) -> Option<(u64, usize)> {
        debug_assert_eq!(self.vli_bits, 0);

        match self.decode(input) {
            VliResult::InvalidVli | VliResult::MoreDataNeeded(_) => None,
            VliResult::Ok(vli, size) => Some((vli, size)),
        }
    }

    /// Decode a vli returning the decoded vli or potentially requesting more data.
    ///
    /// The return value `VliResult::InvalidVli` is a fatal error
    /// and the decoder will not produce correct output anymore until reset.
    pub fn decode(&mut self, input: &[u8]) -> VliResult {
        let mut in_pos = 0;

        #[cfg(debug_assertions)]
        if self.vli_bits == 0 {
            debug_assert_eq!(self.vli, 0);
        }

        while in_pos < input.len() {
            let byte = input[in_pos];
            in_pos += 1;
            self.vli |= u64::from(byte & 0x7f) << self.vli_bits;
            if byte & 0x80 == 0 {
                if byte == 0 && self.vli_bits != 0 {
                    return VliResult::InvalidVli;
                }

                self.vli_bits = 0;
                return VliResult::Ok(mem::take(&mut self.vli), in_pos);
            }
            self.vli_bits += 7;
            if self.vli_bits >= 63 {
                self.vli_bits = 0;
                return VliResult::InvalidVli;
            }
        }
        VliResult::MoreDataNeeded(in_pos)
    }
}

/// Result type of vli decode operation.
pub enum VliResult {
    /// Vli is not valid, this is a fatal error
    InvalidVli,

    /// Vli is valid, (data, size of data consumed in bytes)
    Ok(u64, usize),

    /// Need more data (size of data consumed in bytes)
    MoreDataNeeded(usize),
}
