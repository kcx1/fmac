#[allow(unused_imports)]
use crate::{convert_case, filter_non_hex_chars, format_mac, process_mac};

// CAPITALIZE TEST

#[test]
fn test_capital() {
    let input = "cjaskldufJKLHDUFGYdfnasdkj";
    assert_eq!(convert_case("test", true), "TEST");
    assert_eq!(convert_case(input, true), input.to_uppercase());
}

#[test]
fn test_lowercase() {
    let input = "AJKShfdsdufhwUUDASGF";
    assert_eq!(convert_case("TEST", false), "test");
    assert_eq!(convert_case(input, false), input.to_lowercase());
}

// FILTER SEPARATORS TEST

#[test]
fn test_filter() {
    assert_eq!(filter_non_hex_chars("ab:cd:ef:12:34:56"), "abcdef123456");
    assert_eq!(filter_non_hex_chars("ab-cd-ef-12-34-56"), "abcdef123456");
    assert_eq!(filter_non_hex_chars("ab cd ef 12 34 56"), "abcdef123456");
    assert_eq!(filter_non_hex_chars("ab_cd_ef_12_34_56"), "abcdef123456");
    assert_eq!(filter_non_hex_chars("ab,cd,ef,12,34,56"), "abcdef123456");
    assert_eq!(filter_non_hex_chars("qwertyuiop1234567890"), "e1234567890");
    assert_eq!(
        filter_non_hex_chars("qwertyuiop[]{}1234567890"),
        "e1234567890"
    );
    assert_eq!(
        filter_non_hex_chars("qwertyuiop`~!@#$%^&*()_+-=[]{}1234567890"),
        "e1234567890"
    );
}

// FORMAT MAC TEST

#[test]
fn test_format_mac() {
    let input = "12:34:56:78:9A:BC";
    assert_eq!(format_mac(input, true, "-"), "12-34-56-78-9A-BC");
    assert_eq!(format_mac(input, false, "_"), "12_34_56_78_9a_bc");
    assert_eq!(format_mac(input, false, " "), "12 34 56 78 9a bc");
    assert_eq!(format_mac(input, false, ":"), "12:34:56:78:9a:bc");
    assert_eq!(format_mac(input, false, ""), "123456789abc");
}

// GPT GENERATED TEST

#[test]
fn test_filter_non_hex_chars() {
    let input = "12:34:56:78:9A:BC";
    assert_eq!(filter_non_hex_chars(input), "123456789ABC");

    let input = "1!2@3#4$5%6^7&8*9(0)A>B<C";
    assert_eq!(filter_non_hex_chars(input), "1234567890ABC");
}

#[test]
fn test_convert_case() {
    let input = "12:34:56:78:9a:bc";
    assert_eq!(convert_case(input, true), "12:34:56:78:9A:BC");
    assert_eq!(convert_case(input, false), "12:34:56:78:9a:bc");
}

#[test]
fn test_format_mac_gpt() {
    let input = "12:34:56:78:9A:BC";
    assert_eq!(format_mac(input, true, "-"), "12-34-56-78-9A-BC");
    assert_eq!(format_mac(input, false, ":"), "12:34:56:78:9a:bc");
}

#[test]
fn test_process_mac() {
    // Valid MAC address
    let valid_mac = "12:34:56:78:9A:BC";
    assert_eq!(
        process_mac(valid_mac, false, ":"),
        Ok("12:34:56:78:9a:bc".to_string())
    );

    // Invalid MAC address (short)
    let short_mac = "12:34:56:78:9A";
    assert_eq!(
        process_mac(short_mac, false, ":"),
        Err("Invalid MAC address: 123456789A is missing 2 character(s)".to_string())
    );

    // Invalid MAC address (long)
    let long_mac = "12:34:56:78:9A:BC:DE";
    assert_eq!(
        process_mac(long_mac, false, ":"),
        Err("Invalid MAC address: 123456789ABCDE has 2 extra character(s)".to_string())
    );

    // Invalid MAC address (non-hex characters)
    let non_hex_mac = "12:34:56:78:9A:BC:F!";
    assert_eq!(
        process_mac(non_hex_mac, false, ":"),
        Err("Invalid MAC address: 123456789ABCF has 1 extra character(s)".to_string())
    );
}
