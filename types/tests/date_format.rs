extern crate chrono;
extern crate elastic_types;

use elastic_types::date::*;

#[test]
fn basic_datetime_no_millis() {
	let date = DateTime::<BasicDateTimeNoMillis>::parse("20150703T145502Z").unwrap();

	assert_eq!(
		(2015i32, 7u32, 3u32, 14u32, 55u32, 2u32),
		(
			date.year(), 
			date.month(), 
			date.day(),
			date.hour(), 
			date.minute(), 
			date.second()
		)
	);

	let fmtd = date.format();
	assert_eq!("20150703T145502Z", &fmtd);
}

#[test]
fn basic_date_time() {
	let date = DateTime::<BasicDateTime>::parse("20150703T145502.478Z").unwrap();

	assert_eq!(
		(2015i32, 7u32, 3u32, 14u32, 55u32, 2u32, 478u32),
		(
			date.year(), 
			date.month(), 
			date.day(),
			date.hour(), 
			date.minute(), 
			date.second(),
			date.nanosecond() / 1000000
		)
	);

	let fmtd = date.format();
	assert_eq!("20150703T145502.478Z", &fmtd);
}

#[test]
fn epoch_millis() {
	let date = DateTime::<EpochMillis>::parse("1435935302478").unwrap();

	assert_eq!(
		(2015i32, 7u32, 3u32, 14u32, 55u32, 2u32, 478u32),
		(
			date.year(), 
			date.month(), 
			date.day(),
			date.hour(), 
			date.minute(), 
			date.second(),
			date.nanosecond() / 1000000
		)
	);

	let fmtd = date.format();
	assert_eq!("1435935302478", &fmtd);
}

#[test]
fn epoch_millis_negative() {
	let date = DateTime::<EpochMillis>::parse("-8031171898478").unwrap();

	assert_eq!(
		(1715i32, 7u32, 3u32, 14u32, 55u32, 2u32, 478u32),
		(
			date.year(), 
			date.month(), 
			date.day(),
			date.hour(), 
			date.minute(), 
			date.second(),
			date.nanosecond() / 1000000
		)
	);

	let fmtd = date.format();
	assert_eq!("-8031171898478", &fmtd);
}