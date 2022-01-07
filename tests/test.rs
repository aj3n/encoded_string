use encoded_string::{encoding, Encoded};
use encoding::Gbk;
use serde::Deserialize;

static CASE1: &[u8] = include_bytes!("../testcase/case1.csv");

#[derive(Deserialize)]
struct StructCase1 {
	name: Encoded<Gbk>,
	age: i32,
	gender: Encoded<Gbk>,
}

#[test]
fn test_encoding_string() {
	let mut csv_rdr = csv::ReaderBuilder::new().from_reader(CASE1);
	let mut it = csv_rdr.byte_records();
	while let Some(Ok(rec)) = it.next() {
		let rec: StructCase1 = rec.deserialize(None).unwrap();
		println!("{} {} {}", rec.name, rec.age, rec.gender);
	}
}
