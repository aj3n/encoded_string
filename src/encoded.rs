use crate::encoding::Encoder;
pub struct Encoded<E: Encoder> {
	pub(crate) inner: String,
	pub(crate) _encoder: E,
}

impl<E: Encoder> AsRef<str> for Encoded<E> {
	fn as_ref(&self) -> &str { self.inner.as_ref() }
}

impl<E: Encoder> From<Encoded<E>> for String {
	fn from(s: Encoded<E>) -> Self { s.inner }
}

impl<E: Encoder> From<String> for Encoded<E> {
	fn from(inner: String) -> Self {
		Self {
			inner,
			_encoder: Default::default(),
		}
	}
}

impl<E: Encoder> std::fmt::Display for Encoded<E> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(&self.inner, f)
	}
}
impl<E: Encoder> std::fmt::Debug for Encoded<E> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Debug::fmt(&self.inner, f)
	}
}
