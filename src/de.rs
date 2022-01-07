use crate::{
	encoding::{is_utf8, Encoder},
	Encoded,
};
use serde::{
	de::{Error, Unexpected, Visitor},
	Deserialize, Deserializer,
};
use std::fmt;

struct EncodedVisitor<E: Encoder> {
	marker: std::marker::PhantomData<E>,
}

impl<E: Encoder> EncodedVisitor<E> {
	fn new() -> Self {
		Self {
			marker: std::marker::PhantomData::<E>,
		}
	}
}

impl<'de, Enc: Encoder> Visitor<'de> for EncodedVisitor<Enc> {
	type Value = Encoded<Enc>;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("an encoded string")
	}

	fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
	where
		E: Error,
	{
		let (s, _, has_err) = Enc::to_encoder().decode(v);
		if has_err {
			Err(Error::invalid_type(Unexpected::Bytes(v), &self))
		} else {
			Ok(Self::Value::from(s.into_owned()))
		}
	}
}

impl<'de, E: Encoder> Deserialize<'de> for Encoded<E> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		if is_utf8::<E>() {
			String::deserialize(deserializer).map(Encoded::<E>::from)
		} else {
			deserializer.deserialize_bytes(EncodedVisitor::<E>::new())
		}
	}
}
