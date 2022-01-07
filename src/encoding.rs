pub(crate) use std::any::Any;

pub trait Encoder: Any + Default {
	fn to_encoder() -> &'static encoding_rs::Encoding;
}

#[inline]
pub fn is_utf8<E: Encoder>() -> bool {
	E::default().type_id() == <Utf8 as std::any::Any>::type_id(&Utf8::default())
}

macro_rules! impl_encoding {
	($type_name:ident, $encoder: ident) => {
		#[derive(Clone, Copy, Default)]
		pub struct $type_name();

		impl Encoder for $type_name {
			fn to_encoder() -> &'static encoding_rs::Encoding { encoding_rs::$encoder }
		}
	};
}

impl_encoding!(Utf8, UTF_8);
impl_encoding!(Gbk, GBK);
impl_encoding!(BIG5, BIG5);
impl_encoding!(EucJp, EUC_JP);
impl_encoding!(EucKr, EUC_KR);
impl_encoding!(GB18030, GB18030);
impl_encoding!(IBM866, IBM866);
impl_encoding!(Iso2022Jp, ISO_2022_JP);
impl_encoding!(Iso8859_10, ISO_8859_10);
impl_encoding!(Iso8859_13, ISO_8859_13);
impl_encoding!(Iso8859_14, ISO_8859_14);
impl_encoding!(Iso8859_15, ISO_8859_15);
impl_encoding!(Iso8859_16, ISO_8859_16);
impl_encoding!(Iso8859_2, ISO_8859_2);
impl_encoding!(Iso8859_3, ISO_8859_3);
impl_encoding!(Iso8859_4, ISO_8859_4);
impl_encoding!(Iso8859_5, ISO_8859_5);
impl_encoding!(Iso8859_6, ISO_8859_6);
impl_encoding!(Iso8859_7, ISO_8859_7);
impl_encoding!(Iso8859_8, ISO_8859_8);
impl_encoding!(Iso8859_8I, ISO_8859_8_I);
impl_encoding!(Koi8R, KOI8_R);
impl_encoding!(Koi8U, KOI8_U);
impl_encoding!(MACINTOSH, MACINTOSH);
impl_encoding!(REPLACEMENT, REPLACEMENT);
impl_encoding!(ShiftJis, SHIFT_JIS);
impl_encoding!(Utf16be, UTF_16BE);
impl_encoding!(Utf16le, UTF_16LE);
impl_encoding!(Windows1250, WINDOWS_1250);
impl_encoding!(Windows1251, WINDOWS_1251);
impl_encoding!(Windows1252, WINDOWS_1252);
impl_encoding!(Windows1253, WINDOWS_1253);
impl_encoding!(Windows1254, WINDOWS_1254);
impl_encoding!(Windows1255, WINDOWS_1255);
impl_encoding!(Windows1256, WINDOWS_1256);
impl_encoding!(Windows1257, WINDOWS_1257);
impl_encoding!(Windows1258, WINDOWS_1258);
impl_encoding!(Windows874, WINDOWS_874);
impl_encoding!(XMacCyrillic, X_MAC_CYRILLIC);
impl_encoding!(XUserDefined, X_USER_DEFINED);
