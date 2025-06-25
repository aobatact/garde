use super::util;

#[derive(Debug, garde::Validate)]
struct LengthBoundsTest {
    #[garde(length(5..=10))]
    inclusive_range: String,
    
    #[garde(length(1..100))]
    exclusive_range: String,
    
    #[garde(length(10..))]
    start_only: Vec<i32>,
    
    #[garde(length(..=50))]
    end_only: String,
    
    #[garde(length(bytes, 1..=10))]
    bytes_range: String,
    
    #[garde(length(chars, 1..=5))]
    chars_range: String,
}

#[test]
fn length_bounds_valid() {
    util::check_ok(&[
        LengthBoundsTest {
            inclusive_range: "hello".to_string(),     // 5 chars, within 5..=10
            exclusive_range: "test".to_string(),       // 4 chars, within 1..100
            start_only: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], // 11 items, >= 10
            end_only: "short".to_string(),             // 5 chars, <= 50
            bytes_range: "hello".to_string(),          // 5 bytes, within 1..=10
            chars_range: "world".to_string(),          // 5 chars, within 1..=5
        },
        LengthBoundsTest {
            inclusive_range: "hello!".to_string(),       // 6 chars, within 5..=10
            exclusive_range: "a".to_string(),            // 1 char, within 1..100
            start_only: vec![1; 20],                     // 20 items, >= 10
            end_only: "".to_string(),                    // 0 chars, <= 50
            bytes_range: "a".to_string(),                // 1 byte, within 1..=10
            chars_range: "a".to_string(),                // 1 char, within 1..=5
        },
    ], &())
}

#[test]
fn length_bounds_invalid() {
    util::check_fail!(&[
        LengthBoundsTest {
            inclusive_range: "hi".to_string(),         // 2 chars, less than 5
            exclusive_range: "test".to_string(),       // 4 chars, within range
            start_only: vec![1, 2, 3],                 // 3 items, less than 10
            end_only: "short".to_string(),             // 5 chars, within range
            bytes_range: "hello".to_string(),          // 5 bytes, within range
            chars_range: "world".to_string(),          // 5 chars, within range
        },
        LengthBoundsTest {
            inclusive_range: "hello world!".to_string(), // 12 chars, greater than 10
            exclusive_range: "test".to_string(),          // 4 chars, within range
            start_only: vec![1; 20],                      // 20 items, within range
            end_only: "short".to_string(),                // 5 chars, within range
            bytes_range: "hello".to_string(),             // 5 bytes, within range
            chars_range: "world".to_string(),             // 5 chars, within range
        },
        LengthBoundsTest {
            inclusive_range: "hello".to_string(),      // 5 chars, within range
            exclusive_range: "test".to_string(),       // 4 chars, within range
            start_only: vec![1; 20],                   // 20 items, within range
            end_only: "this is a very long string that exceeds fifty characters total".to_string(), // > 50 chars
            bytes_range: "hello".to_string(),          // 5 bytes, within range
            chars_range: "world".to_string(),          // 5 chars, within range
        },
        LengthBoundsTest {
            inclusive_range: "hello".to_string(),      // 5 chars, within range
            exclusive_range: "test".to_string(),       // 4 chars, within range
            start_only: vec![1; 20],                   // 20 items, within range
            end_only: "short".to_string(),             // 5 chars, within range
            bytes_range: "".to_string(),               // 0 bytes, less than 1
            chars_range: "world".to_string(),          // 5 chars, within range
        },
        LengthBoundsTest {
            inclusive_range: "hello".to_string(),      // 5 chars, within range
            exclusive_range: "test".to_string(),       // 4 chars, within range
            start_only: vec![1; 20],                   // 20 items, within range
            end_only: "short".to_string(),             // 5 chars, within range
            bytes_range: "hello".to_string(),          // 5 bytes, within range
            chars_range: "too many chars".to_string(), // > 5 chars
        },
    ], &())
}

#[cfg(feature = "unicode")]
#[derive(Debug, garde::Validate)]
struct UnicodeRangeTest {
    #[garde(length(graphemes, 1..=2))]
    graphemes: String,
    
    #[garde(length(utf16, 1..=4))]
    utf16: String,
}

#[cfg(feature = "unicode")]
#[test]
fn unicode_length_bounds_valid() {
    util::check_ok(&[
        UnicodeRangeTest {
            graphemes: "😂".to_string(),      // 1 grapheme, within 1..=2
            utf16: "test".to_string(),        // 4 UTF-16 code units, within 1..=4
        },
        UnicodeRangeTest {
            graphemes: "😂😂".to_string(),    // 2 graphemes, within 1..=2
            utf16: "a".to_string(),           // 1 UTF-16 code unit, within 1..=4
        },
    ], &())
}

#[cfg(feature = "unicode")]
#[test]
fn unicode_length_bounds_invalid() {
    util::check_fail!(&[
        UnicodeRangeTest {
            graphemes: "".to_string(),        // 0 graphemes, less than 1
            utf16: "test".to_string(),        // 4 UTF-16 code units, within range
        },
        UnicodeRangeTest {
            graphemes: "😂😂😂".to_string(),  // 3 graphemes, greater than 2
            utf16: "test".to_string(),        // 4 UTF-16 code units, within range
        },
        UnicodeRangeTest {
            graphemes: "😂".to_string(),      // 1 grapheme, within range
            utf16: "tests".to_string(),       // 5 UTF-16 code units, greater than 4
        },
    ], &())
}