/// Casts an usize to an u32 clamping all excess bits off.
#[allow(clippy::cast_possible_truncation)]
pub const fn clamp_us_to_u32(from: usize) -> u32 {
    from as u32
}

/// Casts an u32 to an u16 clamping all excess bits off.
#[allow(clippy::cast_possible_truncation)]
pub const fn clamp_u32_to_u16(from: u32) -> u16 {
    from as u16
}

/// Casts an u32 to an u8 clamping all excess bits off.
#[allow(clippy::cast_possible_truncation)]
pub const fn clamp_u32_to_u8(from: u32) -> u8 {
    from as u8
}

/// Casts an u64 to an u8 clamping all excess bits off.
#[allow(clippy::cast_possible_truncation)]
#[cfg(feature = "bcj")] // only used by bcj filter.
pub const fn clamp_u64_to_u8(from: u64) -> u8 {
    from as u8
}

/// Casts an u64 to an usize clamping all excess bits off.
#[allow(clippy::cast_possible_truncation)]
pub const fn clamp_u64_to_u32(from: u64) -> u32 {
    from as u32
}

/// Casts an u64 to an usize clamping all excess bits off.
#[allow(clippy::cast_possible_truncation)]
#[cfg(feature = "crc64")] //Only used by crc64 validation.
pub const fn clamp_u64_to_us(from: u64) -> usize {
    from as usize
}
