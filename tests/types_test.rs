use ZenithDB::configs::types_config::TypesConfig::{MAX_CHAR_SIZE, MAX_TEXT_SIZE};
use ZenithDB::types::big_int::BIGINT;
use ZenithDB::types::bool::BOOL;
use ZenithDB::types::char::CHAR;
use ZenithDB::types::date_time::DATETIME;
use ZenithDB::types::date::DATE;
use ZenithDB::types::decimal::DECIMAL;
use ZenithDB::types::double::DOUBLE;
use ZenithDB::types::float::FLOAT;
use ZenithDB::types::int::INT;
use ZenithDB::types::tiny_int::TINYINT;
use ZenithDB::types::varchar::VARCHAR;
use ZenithDB::types::time::TIME;
use ZenithDB::types::small_int::SMALLINT;
use ZenithDB::types::text::TEXT;





#[test]
fn test_bigint_basic_operations() {
    // Create from string
    let bigint = BIGINT::new("123456789").expect("Failed to create BIGINT");
    assert_eq!(bigint.value(), 123456789);

    // Serialize to bytes
    let bytes = bigint.to_bytes();
    assert_eq!(bytes.len(), 8);

    // Deserialize from bytes
    let deserialized = BIGINT::from_bytes(&bytes);
    assert_eq!(deserialized.value(), 123456789);
}

#[test]
fn test_bigint_invalid_input() {
    let result = BIGINT::new("not_a_number");
    assert!(result.is_err());
}


#[test]
fn test_bool_true_and_false() {
    // Test creating TRUE
    let b_true = BOOL::new("true").expect("Should parse 'true'");
    assert_eq!(b_true.value(), true); 

    // Test creating FALSE
    let b_false = BOOL::new("false").expect("Should parse 'false'");
    assert_eq!(b_false.value(), false);

    // Round-trip: true
    let true_bytes = b_true.to_bytes();
    let b_from_true = BOOL::from_bytes(&true_bytes);
    assert_eq!(b_from_true.value(), true);

    // Round-trip: false
    let false_bytes = b_false.to_bytes();
    let b_from_false = BOOL::from_bytes(&false_bytes);
    assert_eq!(b_from_false.value(), false);
}

#[test]
fn test_bool_invalid_input() {
    let result = BOOL::new("notabool");
    assert!(result.is_err());
}


#[test]
fn test_char_creation_valid() {
    // Test valid CHAR creation
    let char_value = "Hello";
    let char_size = 10; // Size larger than string length
    let char = CHAR::new(char_size, char_value).expect("Failed to create CHAR");

    assert_eq!(char.value(), char_value);
    assert_eq!(char.size(), char_size);
}

#[test]
fn test_char_creation_invalid_size() {
    // Test CHAR creation with string length exceeding size
    let char_value = "ThisIsTooLong";
    let char_size = 10; // Size is too small for the string
    let result = CHAR::new(char_size, char_value);
    
    assert!(result.is_err());
}

#[test]
fn test_char_to_bytes() {
    // Test converting CHAR to bytes
    let char_value = "Test";
    let char_size = 10;
    let char = CHAR::new(char_size, char_value).expect("Failed to create CHAR");

    let bytes = char.to_bytes();
    
    // Check that the length includes the 4-byte size field + string length + padding
    assert_eq!(bytes.len(), 4 + char_value.len() + (char_size as usize - char_value.len()));
}

#[test]
fn test_char_from_bytes() {
    // Test converting bytes back into CHAR
    let char_value = "Test";
    let char_size = 10;
    let char = CHAR::new(char_size, char_value).expect("Failed to create CHAR");
    
    let bytes = char.to_bytes();
    let char_from_bytes = CHAR::from_bytes(&bytes, char_size).expect("Failed to deserialize CHAR");

    assert_eq!(char_from_bytes.value(), char_value);
    assert_eq!(char_from_bytes.size(), char_size);
}

#[test]
fn test_char_invalid_binary() {
    // Test invalid binary input (too short or corrupted)
    let invalid_bytes = vec![0, 0, 0, 0, 0, 0]; // Invalid length
    let result = CHAR::from_bytes(&invalid_bytes, 10);
    assert!(result.is_err());
}

#[test]
fn test_char_length_overflow() {
    // Test if size exceeds limit
    let char_value = "Overflow";
    let char_size = 5; // Size is too small for the string
    let result = CHAR::new(char_size, char_value);
    
    assert!(result.is_err());
}

#[test]
fn test_datetime_creation_valid() {
    // Test valid DATETIME creation
    let datetime_value = "2025-05-03 12:30:00";
    let datetime = DATETIME::new(datetime_value).expect("Failed to create DATETIME");

    assert_eq!(datetime.value(), datetime_value);
}

#[test]
fn test_datetime_creation_invalid_format() {
    // Test DATETIME creation with invalid format (missing time)
    let datetime_value = "2025-05-03";
    let result = DATETIME::new(datetime_value);
    assert!(result.is_err());

    // Test DATETIME creation with invalid format (extra fields)
    let datetime_value = "2025-05-03 12:30:00 extra";
    let result = DATETIME::new(datetime_value);
    assert!(result.is_err());
}

#[test]
fn test_datetime_to_bytes() {
    // Test converting DATETIME to bytes
    let datetime_value = "2025-05-03 12:30:00";
    let datetime = DATETIME::new(datetime_value).expect("Failed to create DATETIME");

    let bytes = datetime.to_bytes();
    
    // Check that the byte length matches expected size (7 bytes)
    assert_eq!(bytes.len(), 7);
}

#[test]
fn test_datetime_from_bytes() {
    // Test converting bytes back into DATETIME
    let datetime_value = "2025-05-03 12:30:00";
    let datetime = DATETIME::new(datetime_value).expect("Failed to create DATETIME");

    let bytes = datetime.to_bytes();
    let datetime_from_bytes = DATETIME::from_bytes(&bytes);

    assert_eq!(datetime_from_bytes.value(), datetime_value);
}

#[test]
fn test_datetime_invalid_date_or_time() {
    // Test invalid DATETIME with incorrect date or time format
    let result = DATETIME::new("invalid-date-time");
    assert!(result.is_err());
}


#[test]
fn test_date_creation_valid() {
    // Test valid date creation
    let date_value = "2025-05-03";
    let date = DATE::new(date_value).expect("Failed to create DATE");

    assert_eq!(date.value(), date_value);
}

#[test]
fn test_date_creation_invalid_format() {
    // Test date creation with invalid format (incorrect number of segments)
    let date_value = "2025-05";
    let result = DATE::new(date_value);
    assert!(result.is_err());

    // Test date creation with incorrect date
    let date_value = "2025-13-03"; // Invalid month
    let result = DATE::new(date_value);
    assert!(result.is_err());

    let date_value = "2025-05-32"; // Invalid day
    let result = DATE::new(date_value);
    assert!(result.is_err());
}

#[test]
fn test_date_creation_invalid_value() {
    // Test date creation with invalid value (invalid day for month)
    let date_value = "2025-02-30"; // Invalid day for February
    let result = DATE::new(date_value);
    assert!(result.is_err());

    // Test date creation with invalid leap year
    let date_value = "2024-02-29"; // Valid leap year
    let date = DATE::new(date_value).expect("Failed to create DATE");
    assert_eq!(date.value(), date_value);

    let date_value = "2023-02-29"; // Invalid leap year
    let result = DATE::new(date_value);
    assert!(result.is_err());
}

#[test]
fn test_date_to_bytes() {
    // Test converting DATE to bytes
    let date_value = "2025-05-03";
    let date = DATE::new(date_value).expect("Failed to create DATE");

    let bytes = date.to_bytes();
    
    // Check that the byte length matches expected size (4 bytes)
    assert_eq!(bytes.len(), 4);
}

#[test]
fn test_date_from_bytes() {
    // Test converting bytes back into DATE
    let date_value = "2025-05-03";
    let date = DATE::new(date_value).expect("Failed to create DATE");

    let bytes = date.to_bytes();
    let date_from_bytes = DATE::from_bytes(&bytes);

    assert_eq!(date_from_bytes.value(), date_value);
}

#[test]
fn test_date_value_format() {
    // Test the value format for different dates
    let date_value = "2025-05-03";
    let date = DATE::new(date_value).expect("Failed to create DATE");

    let formatted_value = date.value();
    assert_eq!(formatted_value, "2025-05-03");
}



#[test]
fn test_decimal_creation_valid() {
    let decimal = DECIMAL::new("123.45", 5, 2).expect("Failed to create valid DECIMAL");
    assert_eq!(decimal.value(), 12345);
    assert_eq!(decimal.scale(), 2);
    assert_eq!(decimal.precision(), 5);
}

#[test]
fn test_decimal_creation_with_padding() {
    let decimal = DECIMAL::new("123", 5, 2).expect("Failed to create DECIMAL with padding");
    assert_eq!(decimal.value(), 12300);
}

#[test]
fn test_decimal_creation_rounding() {
    let decimal = DECIMAL::new("123.456", 6, 2).expect("Failed to round DECIMAL");
    assert_eq!(decimal.value(), 12346); // Rounded from 12345.6
}

#[test]
fn test_decimal_invalid_format() {
    let result = DECIMAL::new("12.34.56", 6, 2);
    assert!(result.is_err());

    let result = DECIMAL::new("abc", 6, 2);
    assert!(result.is_err());
}

#[test]
fn test_decimal_precision_overflow() {
    let result = DECIMAL::new("123456789012345678901234567890123456789", 39, 0);
    assert!(result.is_err());
}

#[test]
fn test_decimal_scale_greater_than_precision() {
    let result = DECIMAL::new("12.34", 3, 4);
    assert!(result.is_err());
}

#[test]
fn test_decimal_to_from_bytes() {
    let decimal = DECIMAL::new("123.45", 5, 2).unwrap();
    let bytes = decimal.to_bytes();
    let restored = DECIMAL::from_bytes(&bytes, 5, 2);
    assert_eq!(decimal.value(), restored.value());
    assert_eq!(decimal.scale(), restored.scale());
    assert_eq!(decimal.precision(), restored.precision());
}


#[test]
fn test_double_creation_valid() {
    let double = DOUBLE::new("123.456").expect("Valid DOUBLE creation failed");
    assert_eq!(double.value(), 123.456);
}

#[test]
fn test_double_creation_invalid() {
    let result = DOUBLE::new("not_a_number");
    assert!(result.is_err());
}

#[test]
fn test_double_to_bytes_and_from_bytes() {
    let original = DOUBLE::new("3.14159").unwrap();
    let bytes = original.to_bytes();
    let restored = DOUBLE::from_bytes(&bytes);
    assert!((original.value() - restored.value()).abs() < f64::EPSILON);
}


#[test]
fn test_float_creation_valid() {
    let float = FLOAT::new("123.45").expect("Valid FLOAT creation failed");
    assert!((float.value() - 123.45).abs() < f32::EPSILON);
}

#[test]
fn test_float_creation_invalid() {
    let result = FLOAT::new("abc");
    assert!(result.is_err());
}

#[test]
fn test_float_to_bytes_and_from_bytes() {
    let original = FLOAT::new("5.25").unwrap();
    let bytes = original.to_bytes();
    let restored = FLOAT::from_bytes(&bytes);
    assert!((original.value() - restored.value()).abs() < f32::EPSILON);
}


#[test]
fn test_int_creation_valid() {
    let int = INT::new("42").expect("Valid INT creation failed");
    assert_eq!(int.value(), 42);
}

#[test]
fn test_int_creation_invalid() {
    let result = INT::new("not_a_number");
    assert!(result.is_err());
}

#[test]
fn test_int_to_bytes_and_from_bytes() {
    let original = INT::new("123456").unwrap();
    let bytes = original.to_bytes();
    let restored = INT::from_bytes(&bytes);
    assert_eq!(original.value(), restored.value());
}


#[test]
fn test_smallint_creation_valid() {
    let smallint = SMALLINT::new("32767").expect("Valid SMALLINT creation failed");
    assert_eq!(smallint.value(), 32767);
}

#[test]
fn test_smallint_creation_invalid() {
    let result = SMALLINT::new("not_a_number");
    assert!(result.is_err());
}

#[test]
fn test_smallint_to_bytes_and_from_bytes() {
    let original = SMALLINT::new("-12345").unwrap();
    let bytes = original.to_bytes();
    let restored = SMALLINT::from_bytes(&bytes);
    assert_eq!(original.value(), restored.value());
}


#[test]
fn test_text_creation_valid() {
    let txt = TEXT::new("This is valid").expect("Should create TEXT");
    assert_eq!(txt.value(), "This is valid");
}

#[test]
fn test_text_creation_invalid_length() {
    let oversized = "a".repeat((MAX_TEXT_SIZE + 1) as usize);
    let result = TEXT::new(&oversized);
    assert!(result.is_err());
}

#[test]
fn test_text_to_bytes_and_from_bytes() {
    let original = TEXT::new("serialize me").unwrap();
    let bytes = original.to_bytes();
    let restored = TEXT::from_bytes(&bytes).expect("Should decode from bytes");
    assert_eq!(original.value(), restored.value());
}


#[test]
fn test_time_creation_valid() {
    let time = TIME::new("23:59:59").unwrap();
    assert_eq!(time.value(), "23:59:59");
}

#[test]
fn test_time_creation_invalid_format() {
    let err = TIME::new("24-00-00");
    assert!(err.is_err());
}

#[test]
fn test_time_serialization_roundtrip() {
    let time = TIME::new("05:04:03").unwrap();
    let bytes = time.to_bytes();
    let parsed = TIME::from_bytes(&bytes);
    assert_eq!(parsed.value(), "05:04:03");
}


#[test]
fn test_tinyint_creation_valid() {
    let num = TINYINT::new("127").unwrap();
    assert_eq!(num.value(), 127);
}

#[test]
fn test_tinyint_creation_invalid() {
    let num = TINYINT::new("200"); // out of i8 range
    assert!(num.is_err());
}

#[test]
fn test_tinyint_serialization_roundtrip() {
    let num = TINYINT::new("-5").unwrap();
    let bytes = num.to_bytes();
    let parsed = TINYINT::from_bytes(&bytes);
    assert_eq!(parsed.value(), -5);
}


#[test]
fn test_varchar_creation_valid() {
    let v = VARCHAR::new(20, "hello").unwrap();
    assert_eq!(v.value(), "hello");
}

#[test]
fn test_varchar_creation_too_long() {
    let long_str = "a".repeat((MAX_CHAR_SIZE + 1) as usize);
    let res = VARCHAR::new(MAX_CHAR_SIZE + 1, &long_str);
    assert!(res.is_err());
}

#[test]
fn test_varchar_serialization_roundtrip() {
    let original = VARCHAR::new(20, "gen_z_rustacean").unwrap();
    let bytes = VARCHAR::to_bytes(&original);
    let restored = VARCHAR::from_bytes(&bytes, 20).unwrap();
    assert_eq!(original.value(), restored.value());
}

