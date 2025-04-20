use crate::decoder::XzError;
use sha2::{Digest, Sha256};

/// Wrapper for the sha2 Sha256 type.
#[derive(Clone, Default, Debug)]
#[repr(transparent)]
pub struct XzSha256 {
    /// Holds the SHA256 state, must be an option because sha2 does not include a const constructor.
    delegate: Option<Sha256>,
}

impl XzSha256 {
    ///Constructor.
    pub(crate) const fn new() -> Self {
        Self { delegate: None }
    }

    /// Unwraps or initializes the inner value from the sha2 crate.
    fn delegate(&mut self) -> &mut Sha256 {
        if let Some(ref mut delegate) = self.delegate {
            return delegate;
        }

        self.delegate = Some(Sha256::new());
        self.delegate.as_mut().expect("cannot fail")
    }

    /// Retries the inner value from the sha2 crate leaving this struct uninitialized.
    fn take_delegate(&mut self) -> Sha256 {
        self.delegate.take().unwrap_or_default()
    }

    /// Resets the inner state to uninitialized.
    pub const fn reset(&mut self) {
        self.delegate = None;
    }

    /// Update the digest with some data.
    pub fn update(&mut self, buf: &[u8]) {
        Digest::update(self.delegate(), buf);
    }

    /// Validate the digest.
    pub fn validate(&mut self, buf: &[u8]) -> Result<(), XzError> {
        let state = self.take_delegate();
        let binding = state.finalize();
        let actual = binding.as_slice();
        if buf != actual {
            let actual: [u8; 32] = actual.try_into().map_err(|_| XzError::CorruptedData)?; //ERR is impossible
            let expected: [u8; 32] = buf.try_into().map_err(|_| XzError::CorruptedData)?; //ERR should be impossible
            return Err(XzError::ContentSha256Mismatch(actual, expected));
        }
        Ok(())
    }
}
