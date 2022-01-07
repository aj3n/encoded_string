use encoded_string::{encoding, Encoded};
use encoding::Gbk;
use serde::Deserialize;

static CASE1_CSV: &[u8] = include_bytes!("../testcase/case1.csv");
static CASE1_JSON: &[u8] = include_bytes!("../testcase/case1.json");

#[derive(Debug,Deserialize)]
struct StructCase1 {
	name: Encoded<Gbk>,
	age: i32,
	gender: Encoded<Gbk>,
}

#[test]
fn test_encoding_string() {
	let mut csv_rdr = csv::ReaderBuilder::new().from_reader(CASE1_CSV);
	let mut it = csv_rdr.byte_records();
	while let Some(Ok(rec)) = it.next() {
		let rec: StructCase1 = rec.deserialize(None).unwrap();
		dbg!(rec);
	}

	let rec: StructCase1 = dbg!(serde_json::from_slice(CASE1_JSON).unwrap());
}
