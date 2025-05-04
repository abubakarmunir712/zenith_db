use ZenithDB::configs::types_config::TypesConfig::{MAX_TEXT_SIZE, MIN_TEXT_SIZE};
use ZenithDB::enums::datatypes::DataType;
use ZenithDB::enums::type_errors::*;
use ZenithDB::enums::typed_value::TypedValue;
use ZenithDB::storage::catalog::entries::column_entry::ColumnEntry;
use ZenithDB::types::big_int::BIGINT;
use ZenithDB::types::bool::BOOL;
use ZenithDB::types::char::CHAR;
use ZenithDB::types::date::DATE;
use ZenithDB::types::date_time::DATETIME;
use ZenithDB::types::decimal::DECIMAL;
use ZenithDB::types::double::DOUBLE;
use ZenithDB::types::float::FLOAT;
use ZenithDB::types::int::INT;
use ZenithDB::types::small_int::SMALLINT;
use ZenithDB::types::text::TEXT;
use ZenithDB::types::time::TIME;
use ZenithDB::types::tiny_int::TINYINT;
use ZenithDB::types::varchar::VARCHAR;

// -------------- For INT data type -----------------------

#[test]
fn test_int_new_valid() {
    let int = INT::new("123");
    assert!(int.is_ok());
    assert_eq!(int.unwrap().value(), 123);
}

#[test]
fn test_int_new_invalid_format() {
    let int = INT::new("abc");
    assert!(int.is_err());
    assert_eq!(int.unwrap_err(), NumericError::InvalidFormat.message());
}

#[test]
fn test_int_new_out_of_range() {
    let val = i64::MAX.to_string(); // exceeds i32 range
    let int = INT::new(&val);
    assert!(int.is_err());
    assert_eq!(int.unwrap_err(), NumericError::OutOfRange.message());
}

#[test]
fn test_int_to_from_bytes() {
    let int = INT::new("456").unwrap();
    let bytes = int.to_bytes();
    let restored = INT::from_bytes(&bytes);
    assert_eq!(int.value(), restored.value());
}

// -------------- For BIGINT data type -----------------------

#[test]
fn test_bigint_new_valid() {
    let bigint = BIGINT::new("1234567890123");
    assert!(bigint.is_ok());
    assert_eq!(bigint.unwrap().value(), 1234567890123);
}

#[test]
fn test_bigint_new_invalid_format() {
    let bigint = BIGINT::new("notanumber");
    assert!(bigint.is_err());
    assert_eq!(bigint.unwrap_err(), NumericError::InvalidFormat.message());
}

#[test]
fn test_bigint_new_out_of_range() {
    let val = "9223372036854775807123"; // just beyond i64 max
    let bigint = BIGINT::new(val);
    assert!(bigint.is_err());
    assert_eq!(bigint.unwrap_err(), NumericError::OutOfRange.message());
}

#[test]
fn test_bigint_to_from_bytes() {
    let bigint = BIGINT::new("-9999999999").unwrap();
    let bytes = bigint.to_bytes();
    let restored = BIGINT::from_bytes(&bytes);
    assert_eq!(bigint.value(), restored.value());
}

// -------------- For SMALLINT data type -----------------------

#[test]
fn test_smallint_new_valid() {
    let smallint = SMALLINT::new("32767");
    assert!(smallint.is_ok());
    assert_eq!(smallint.unwrap().value(), 32767);
}

#[test]
fn test_smallint_new_invalid_format() {
    let smallint = SMALLINT::new("hello");
    assert!(smallint.is_err());
    assert_eq!(smallint.unwrap_err(), NumericError::InvalidFormat.message());
}

#[test]
fn test_smallint_new_out_of_range() {
    let val = "40000"; // exceeds i16
    let smallint = SMALLINT::new(val);
    assert!(smallint.is_err());
    assert_eq!(smallint.unwrap_err(), NumericError::OutOfRange.message());
}

#[test]
fn test_smallint_to_from_bytes() {
    let smallint = SMALLINT::new("-1234").unwrap();
    let bytes = smallint.to_bytes();
    let restored = SMALLINT::from_bytes(&bytes);
    assert_eq!(smallint.value(), restored.value());
}

// -------------- For TINYINT data type -----------------------

#[test]
fn test_tinyint_new_valid() {
    let tinyint = TINYINT::new("127");
    assert!(tinyint.is_ok());
    assert_eq!(tinyint.unwrap().value(), 127);
}

#[test]
fn test_tinyint_new_invalid_format() {
    let tinyint = TINYINT::new("NaN");
    assert!(tinyint.is_err());
    assert_eq!(tinyint.unwrap_err(), NumericError::InvalidFormat.message());
}

#[test]
fn test_tinyint_new_out_of_range() {
    let val = "200"; // exceeds i8
    let tinyint = TINYINT::new(val);
    assert!(tinyint.is_err());
    assert_eq!(tinyint.unwrap_err(), NumericError::OutOfRange.message());
}

#[test]
fn test_tinyint_to_from_bytes() {
    let tinyint = TINYINT::new("-128").unwrap();
    let bytes = tinyint.to_bytes();
    let restored = TINYINT::from_bytes(&bytes);
    assert_eq!(tinyint.value(), restored.value());
}

// ---------------- For BOOL data type ----------------

#[test]
fn test_bool_true() {
    let b = BOOL::new("true").unwrap();
    assert_eq!(b.value(), true);
    assert_eq!(b.to_bytes(), vec![1]);
}

#[test]
fn test_bool_false() {
    let b = BOOL::new("false").unwrap();
    assert_eq!(b.value(), false);
    assert_eq!(b.to_bytes(), vec![0]);
}

#[test]
fn test_bool_invalid_input() {
    assert!(BOOL::new("maybe").is_err());
}

#[test]
fn test_bool_from_bytes_true() {
    let b = BOOL::from_bytes(&[1]);
    assert_eq!(b.value(), true);
}

#[test]
fn test_bool_from_bytes_false() {
    let b = BOOL::from_bytes(&[0]);
    assert_eq!(b.value(), false);
}

// ------ DATE Tests ------

#[test]
fn test_valid_date_creation() {
    let date = DATE::new("2024-04-25").unwrap();
    assert_eq!(date.year(), 2024);
    assert_eq!(date.month(), 4);
    assert_eq!(date.day(), 25);
}

#[test]
fn test_invalid_format_date() {
    let result = DATE::new("2024/04/25");
    assert!(result.is_err());
}

#[test]
fn test_invalid_date_values() {
    let result = DATE::new("2024-02-30"); // Feb 30 is invalid
    assert!(result.is_err());
}

#[test]
fn test_leap_year_true() {
    assert!(DATE::is_year_leap(2024));
}

#[test]
fn test_leap_year_false() {
    assert!(!DATE::is_year_leap(2023));
}

#[test]
fn test_to_and_from_bytes_date() {
    let date = DATE::new("2024-12-31").unwrap();
    let bytes = date.to_bytes();
    let from = DATE::from_bytes(&bytes);
    assert_eq!(from.year(), 2024);
    assert_eq!(from.month(), 12);
    assert_eq!(from.day(), 31);
}

#[test]
fn test_date_value_string() {
    let date = DATE::new("2024-04-09").unwrap();
    assert_eq!(date.value(), "2024-04-09");
}

// ------ TIME Tests ------

#[test]
fn test_valid_time_creation() {
    let time = TIME::new("13:45:30").unwrap();
    assert_eq!(time.hours(), 13);
    assert_eq!(time.minutes(), 45);
    assert_eq!(time.seconds(), 30);
}

#[test]
fn test_invalid_time_format() {
    let result = TIME::new("134530");
    assert!(result.is_err());
}

#[test]
fn test_invalid_time_values() {
    let result = TIME::new("25:00:00");
    assert!(result.is_err());
}

#[test]
fn test_to_and_from_bytes_time() {
    let time = TIME::new("12:34:56").unwrap();
    let bytes = time.to_bytes();
    let from = TIME::from_bytes(&bytes);
    assert_eq!(from.hours(), 12);
    assert_eq!(from.minutes(), 34);
    assert_eq!(from.seconds(), 56);
}

#[test]
fn test_time_value_string() {
    let time = TIME::new("04:09:08").unwrap();
    assert_eq!(time.value(), "04:09:08");
}

// ------ DATETIME Tests ------

#[test]
fn test_valid_datetime_creation() {
    let dt = DATETIME::new("2024-04-25 13:45:30").unwrap();
    assert_eq!(dt.date().year(), 2024);
    assert_eq!(dt.time().minutes(), 45);
}

#[test]
fn test_invalid_datetime_format() {
    let result = DATETIME::new("2024-04-25T13:45:30");
    assert!(result.is_err());
}

#[test]
fn test_to_and_from_bytes_datetime() {
    let dt = DATETIME::new("2024-04-25 13:45:30").unwrap();
    let bytes = dt.to_bytes();
    let from = DATETIME::from_bytes(&bytes);
    assert_eq!(from.value(), "2024-04-25 13:45:30");
}

#[test]
fn test_datetime_value_string() {
    let dt = DATETIME::new("2024-04-25 01:02:03").unwrap();
    assert_eq!(dt.value(), "2024-04-25 01:02:03");
}

// ------ DOUBLE Tests ------

#[test]
fn test_valid_double_creation() {
    let double = DOUBLE::new("3.1415").unwrap();
    assert_eq!(double.value(), 3.1415);
}

#[test]
fn test_invalid_double_format() {
    let result = DOUBLE::new("abc");
    assert!(result.is_err());
}

#[test]
fn test_double_nan_infinite() {
    assert!(DOUBLE::new("NaN").is_err());
    assert!(DOUBLE::new("inf").is_err());
}

#[test]
fn test_double_to_and_from_bytes() {
    let double = DOUBLE::new("42.42").unwrap();
    let bytes = double.to_bytes();
    let restored = DOUBLE::from_bytes(&bytes);
    assert_eq!(restored.value(), 42.42);
}

// ------ FLOAT Tests ------

#[test]
fn test_valid_float_creation() {
    let float = FLOAT::new("2.718").unwrap();
    assert_eq!(float.value(), 2.718);
}

#[test]
fn test_invalid_float_format() {
    let result = FLOAT::new("xyz");
    assert!(result.is_err());
}

#[test]
fn test_float_nan_infinite() {
    assert!(FLOAT::new("NaN").is_err());
    assert!(FLOAT::new("inf").is_err());
}

#[test]
fn test_float_to_and_from_bytes() {
    let float = FLOAT::new("15.75").unwrap();
    let bytes = float.to_bytes();
    let restored = FLOAT::from_bytes(&bytes);
    assert_eq!(restored.value(), 15.75);
}

// ------ DECIMAL Tests ------

#[test]
fn test_valid_decimal_creation() {
    let decimal = DECIMAL::new("123.45", 5, 2).unwrap();
    assert_eq!(decimal.value(), 12345);
    assert_eq!(decimal.value_string(), "123.45");
}

// ------

#[test]
fn test_decimal_with_padding_zeros() {
    let decimal = DECIMAL::new("12.3", 5, 2).unwrap();
    assert_eq!(decimal.value(), 1230);
    assert_eq!(decimal.value_string(), "12.30");
}

// ------

#[test]
fn test_decimal_invalid_format_alpha() {
    let result = DECIMAL::new("12.a3", 5, 2);
    assert!(result.is_err());
}

// ------

#[test]
fn test_decimal_precision_overflow() {
    let result = DECIMAL::new("12345.67", 6, 2); // Should overflow precision
    assert!(result.is_err());
}

// ------

#[test]
fn test_decimal_rounding_overflow() {
    let result = DECIMAL::new("999.999", 5, 2); // Round-up might overflow
    assert!(result.is_err());
}

// ------

#[test]
fn test_decimal_to_and_from_bytes() {
    let original = DECIMAL::new("456.78", 6, 2).unwrap();
    let bytes = original.to_bytes();
    let restored = DECIMAL::from_bytes(&bytes, 6, 2);
    assert_eq!(restored.value(), 45678);
    assert_eq!(restored.value_string(), "456.78");
}

// ------

#[test]
fn test_negative_decimal_value() {
    let decimal = DECIMAL::new("-123.45", 6, 2).unwrap();
    assert_eq!(decimal.value(), -12345);
    assert_eq!(decimal.value_string(), "-123.45");
}

// ------

#[test]
fn test_negative_with_padding_zeros() {
    let decimal = DECIMAL::new("-7.1", 4, 2).unwrap();
    assert_eq!(decimal.value(), -710);
    assert_eq!(decimal.value_string(), "-7.10");
}

// ------

#[test]
fn test_negative_decimal_rounding_overflow() {
    let result = DECIMAL::new("-999.999", 5, 2); // Should overflow after rounding
    assert!(result.is_err());
}

#[test]
fn test_char_value_within_size() {
    let char_field = CHAR::new(10, "Hello").unwrap();
    assert_eq!(char_field.value(), "Hello");
    assert_eq!(char_field.size(), 10);
    assert_eq!(char_field.to_bytes().len(), 14); // 4 bytes for length + 5 bytes for value + 5 bytes for padding
}

#[test]
fn test_char_value_exceeds_size() {
    let result = CHAR::new(5, "Hello, World!");
    assert!(result.is_err());
}

#[test]
fn test_char_value_with_padding_zeros() {
    let char_field = CHAR::new(10, "Hi").unwrap();
    assert_eq!(char_field.value(), "Hi");
    assert_eq!(char_field.size(), 10);
    assert_eq!(char_field.to_bytes().len(), 14); // 4 bytes for length + 2 bytes for value + 8 bytes for padding
}

// ------ VARCHAR Tests ------

#[test]
fn test_varchar_value_within_size() {
    let varchar_field = VARCHAR::new(10, "Hello").unwrap();
    assert_eq!(varchar_field.value(), "Hello");
    assert_eq!(varchar_field.size(), 10);
    assert_eq!(varchar_field.to_bytes().len(), 5); // No padding for VARCHAR
}

#[test]
fn test_varchar_value_exceeds_size() {
    let result = VARCHAR::new(5, "Hello, World!");
    assert!(result.is_err());
}

#[test]
fn test_varchar_value_below_min_size() {
    let result = VARCHAR::new(5, "");
    assert!(result.is_err());
}

// ------ TEXT Tests ------

#[test]
fn test_text_value_within_size() {
    let text_field = TEXT::new("Hello, World!").unwrap();
    assert_eq!(text_field.value(), "Hello, World!");
    assert_eq!(text_field.to_bytes().len(), 17); // 4 bytes for length + 14 bytes for value
}

#[test]
fn test_text_value_exceeds_size() {
    let str = &"A".repeat(MAX_TEXT_SIZE as usize + 1);
    let result = TEXT::new(str);
    assert!(result.is_err());
}

#[test]
fn test_text_value_below_min_size() {
    let str = &"A".repeat(MIN_TEXT_SIZE as usize - 1);
    let result = TEXT::new(str);
    assert!(result.is_err());
}

// ------ Edge case tests ------

#[test]
fn test_char_empty_value() {
    let char_field = CHAR::new(10, "");
    assert!(char_field.is_err());
}

#[test]
fn test_varchar_empty_value() {
    let varchar_field = VARCHAR::new(10, " ").unwrap();
    assert_eq!(varchar_field.value(), " ");
    assert_eq!(varchar_field.size(), 10);
    assert_eq!(varchar_field.to_bytes().len(), 1); // No bytes for value
}

#[test]
fn test_text_empty_value() {
    let text_field = TEXT::new(" ").unwrap();
    assert_eq!(text_field.value(), " ");
    assert_eq!(text_field.to_bytes().len(), 5); // 4 bytes for length + 1 bytes for value
}

#[test]
fn test_varchar_padding_zero_value() {
    let varchar_field = VARCHAR::new(5, "Hi").unwrap();
    assert_eq!(varchar_field.value(), "Hi");
    assert_eq!(varchar_field.size(), 5);
    assert_eq!(varchar_field.to_bytes().len(), 2); // 2 bytes for value, no padding for VARCHAR
}

// Helper function to create a ColumnEntry for testing
fn create_column_entry(
    datatype: DataType,
    max_size: u32,
    null: bool,
    unique: bool,
    is_primary_key: bool,
    is_foreign_key: bool,
) -> ColumnEntry {
    ColumnEntry {
        column_name: "test_column".to_string(),
        oid: 1,
        datatype,
        max_size,
        null,
        unique,
        is_primary_key,
        is_foreign_key,
    }
    
}

// Test serialization and deserialization for each TypedValue variant
#[test]
fn test_typed_value_char() {
    let char_val = CHAR::new(10, "Hello").unwrap();
    let typed_value = TypedValue::CHAR(char_val);
    let bytes = typed_value.to_bytes();
    let column_entry = create_column_entry(DataType::CHAR, 10, true, false, false, false);
    let restored = TypedValue::from_bytes(&bytes, &column_entry);
    match restored {
        TypedValue::CHAR(c) => assert_eq!(c.value(), "Hello"),
        _ => panic!("Expected CHAR variant"),
    }
}

#[test]
fn test_typed_value_varchar() {
    let varchar_val = VARCHAR::new(10, "Hello").unwrap();
    let typed_value = TypedValue::VARCHAR(varchar_val);
    let bytes = typed_value.to_bytes();
    let column_entry = create_column_entry(DataType::VARCHAR, 10, true, false, false, false);
    let restored = TypedValue::from_bytes(&bytes, &column_entry);
    match restored {
        TypedValue::VARCHAR(v) => assert_eq!(v.value(), "Hello"),
        _ => panic!("Expected VARCHAR variant"),
    }
}

#[test]
fn test_typed_value_bool() {
    let bool_val = BOOL::new("true").unwrap();
    let typed_value = TypedValue::BOOL(bool_val);
    let bytes = typed_value.to_bytes();
    let column_entry = create_column_entry(DataType::BOOL, 1, true, false, false, false);
    let restored = TypedValue::from_bytes(&bytes, &column_entry);
    match restored {
        TypedValue::BOOL(b) => assert_eq!(b.value(), true),
        _ => panic!("Expected BOOL variant"),
    }
}

#[test]
fn test_typed_value_int() {
    let int_val = INT::new("123").unwrap();
    let typed_value = TypedValue::INT(int_val);
    let bytes = typed_value.to_bytes();
    let column_entry = create_column_entry(DataType::INT, 4, true, false, false, false);
    let restored = TypedValue::from_bytes(&bytes, &column_entry);
    match restored {
        TypedValue::INT(i) => assert_eq!(i.value(), 123),
        _ => panic!("Expected INT variant"),
    }
}

#[test]
fn test_typed_value_bigint() {
    let bigint_val = BIGINT::new("1234567890123").unwrap();
    let typed_value = TypedValue::BIGINT(bigint_val);
    let bytes = typed_value.to_bytes();
    let column_entry = create_column_entry(DataType::BIGINT, 8, true, false, false, false);
    let restored = TypedValue::from_bytes(&bytes, &column_entry);
    match restored {
        TypedValue::BIGINT(b) => assert_eq!(b.value(), 1234567890123),
        _ => panic!("Expected BIGINT variant"),
    }
}

#[test]
fn test_typed_value_smallint() {
    let smallint_val = SMALLINT::new("32767").unwrap();
    let typed_value = TypedValue::SMALLINT(smallint_val);
    let bytes = typed_value.to_bytes();
    let column_entry = create_column_entry(DataType::SMALLINT, 2, true, false, false, false);
    let restored = TypedValue::from_bytes(&bytes, &column_entry);
    match restored {
        TypedValue::SMALLINT(s) => assert_eq!(s.value(), 32767),
        _ => panic!("Expected SMALLINT variant"),
    }
}

#[test]
fn test_typed_value_tinyint() {
    let tinyint_val = TINYINT::new("127").unwrap();
    let typed_value = TypedValue::TINYINT(tinyint_val);
    let bytes = typed_value.to_bytes();
    let column_entry = create_column_entry(DataType::TINYINT, 1, true, false, false, false);
    let restored = TypedValue::from_bytes(&bytes, &column_entry);
    match restored {
        TypedValue::TINYINT(t) => assert_eq!(t.value(), 127),
        _ => panic!("Expected TINYINT variant"),
    }
}

#[test]
fn test_typed_value_decimal() {
    let decimal_val = DECIMAL::new("123.45", 5, 2).unwrap();
    let typed_value = TypedValue::DECIMAL(decimal_val);
    let bytes = typed_value.to_bytes();
    let column_entry = create_column_entry(DataType::DECIMAL, 502, true, false, false, false);
    let restored = TypedValue::from_bytes(&bytes, &column_entry);
    match restored {
        TypedValue::DECIMAL(d) => assert_eq!(d.value_string(), "123.45"),
        _ => panic!("Expected DECIMAL variant"),
    }
}

#[test]
fn test_typed_value_double() {
    let double_val = DOUBLE::new("3.1415").unwrap();
    let typed_value = TypedValue::DOUBLE(double_val);
    let bytes = typed_value.to_bytes();
    let column_entry = create_column_entry(DataType::DOUBLE, 8, true, false, false, false);
    let restored = TypedValue::from_bytes(&bytes, &column_entry);
    match restored {
        TypedValue::DOUBLE(d) => assert_eq!(d.value(), 3.1415),
        _ => panic!("Expected DOUBLE variant"),
    }
}

#[test]
fn test_typed_value_float() {
    let float_val = FLOAT::new("2.718").unwrap();
    let typed_value = TypedValue::FLOAT(float_val);
    let bytes = typed_value.to_bytes();
    let column_entry = create_column_entry(DataType::FLOAT, 4, true, false, false, false);
    let restored = TypedValue::from_bytes(&bytes, &column_entry);
    match restored {
        TypedValue::FLOAT(f) => assert_eq!(f.value(), 2.718),
        _ => panic!("Expected FLOAT variant"),
    }
}

#[test]
fn test_typed_value_date() {
    let date_val = DATE::new("2024-04-25").unwrap();
    let typed_value = TypedValue::DATE(date_val);
    let bytes = typed_value.to_bytes();
    let column_entry = create_column_entry(DataType::DATE, 12, true, false, false, false);
    let restored = TypedValue::from_bytes(&bytes, &column_entry);
    match restored {
        TypedValue::DATE(d) => assert_eq!(d.value(), "2024-04-25"),
        _ => panic!("Expected DATE variant"),
    }
}

#[test]
fn test_typed_value_time() {
    let time_val = TIME::new("13:45:30").unwrap();
    let typed_value = TypedValue::TIME(time_val);
    let bytes = typed_value.to_bytes();
    let column_entry = create_column_entry(DataType::TIME, 8, true, false, false, false);
    let restored = TypedValue::from_bytes(&bytes, &column_entry);
    match restored {
        TypedValue::TIME(t) => assert_eq!(t.value(), "13:45:30"),
        _ => panic!("Expected TIME variant"),
    }
}

#[test]
fn test_typed_value_datetime() {
    let datetime_val = DATETIME::new("2024-04-25 13:45:30").unwrap();
    let typed_value = TypedValue::DATETIME(datetime_val);
    let bytes = typed_value.to_bytes();
    let column_entry = create_column_entry(DataType::DATETIME, 20, true, false, false, false);
    let restored = TypedValue::from_bytes(&bytes, &column_entry);
    match restored {
        TypedValue::DATETIME(dt) => assert_eq!(dt.value(), "2024-04-25 13:45:30"),
        _ => panic!("Expected DATETIME variant"),
    }
}

#[test]
fn test_typed_value_text() {
    let text_val = TEXT::new("Hello, World!").unwrap();
    let typed_value = TypedValue::TEXT(text_val);
    let bytes = typed_value.to_bytes();
    let column_entry =
        create_column_entry(DataType::TEXT, MAX_TEXT_SIZE, true, false, false, false);
    let restored = TypedValue::from_bytes(&bytes, &column_entry);
    match restored {
        TypedValue::TEXT(t) => assert_eq!(t.value(), "Hello, World!"),
        _ => panic!("Expected TEXT variant"),
    }
}


// Edge case: TEXT at max size
#[test]
fn test_typed_value_text_max_size() {
    let text_str = "A".repeat(MAX_TEXT_SIZE as usize);
    let text_val = TEXT::new(&text_str).unwrap();
    let typed_value = TypedValue::TEXT(text_val);
    let bytes = typed_value.to_bytes();
    let column_entry =
        create_column_entry(DataType::TEXT, MAX_TEXT_SIZE, true, false, false, false);
    let restored = TypedValue::from_bytes(&bytes, &column_entry);
    match restored {
        TypedValue::TEXT(t) => assert_eq!(t.value(), text_str),
        _ => panic!("Expected TEXT variant"),
    }
}

// Edge case: DECIMAL with negative value
#[test]
fn test_typed_value_decimal_negative() {
    let decimal_val = DECIMAL::new("-123.45", 5, 2).unwrap();
    let typed_value = TypedValue::DECIMAL(decimal_val);
    let bytes = typed_value.to_bytes();
    let column_entry = create_column_entry(DataType::DECIMAL, 502, true, false, false, false);
    let restored = TypedValue::from_bytes(&bytes, &column_entry);
    match restored {
        TypedValue::DECIMAL(d) => assert_eq!(d.value_string(), "-123.45"),
        _ => panic!("Expected DECIMAL variant"),
    }
}

