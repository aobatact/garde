use garde::{Validate, ErrorCode, StandardErrorCode};

#[derive(Validate)]
struct ComprehensiveTest {
    #[garde(email)]
    #[garde(code("CUSTOM_EMAIL"))]
    email: String,
    
    #[garde(url)]
    #[garde(code("CUSTOM_URL"))]
    url: String,
    
    #[garde(phone_number)]
    #[garde(code("CUSTOM_PHONE"))]
    phone: String,
    
    #[garde(credit_card)]
    #[garde(code("CUSTOM_CC"))]
    credit_card: String,
    
    #[garde(ip)]
    #[garde(code("CUSTOM_IP"))]
    ip: String,
    
    #[garde(ascii)]
    #[garde(code("CUSTOM_ASCII"))]
    ascii_field: String,
    
    #[garde(alphanumeric)]
    #[garde(code("CUSTOM_ALPHANUM"))]
    alphanum_field: String,
    
    #[garde(contains("test"))]
    #[garde(code("CUSTOM_CONTAINS"))]
    contains_field: String,
    
    #[garde(prefix("pre_"))]
    #[garde(code("CUSTOM_PREFIX"))]
    prefix_field: String,
    
    #[garde(suffix("_suf"))]
    #[garde(code("CUSTOM_SUFFIX"))]
    suffix_field: String,
    
    #[garde(pattern(r"^[a-z]+$"))]
    #[garde(code("CUSTOM_PATTERN"))]
    pattern_field: String,
    
    #[garde(length(5..=10))]
    #[garde(code("CUSTOM_LENGTH"))]
    length_field: String,
}

#[test]
fn test_comprehensive_custom_error_codes() {
    let test = ComprehensiveTest {
        email: "invalid-email".to_string(),
        url: "invalid-url".to_string(),
        phone: "invalid-phone".to_string(),
        credit_card: "invalid-cc".to_string(),
        ip: "invalid-ip".to_string(),
        ascii_field: "非ASCII".to_string(),
        alphanum_field: "not@alphanum".to_string(),
        contains_field: "no-match".to_string(),
        prefix_field: "wrong_prefix".to_string(),
        suffix_field: "wrong_suffix".to_string(),
        pattern_field: "123ABC".to_string(),
        length_field: "ab".to_string(), // too short
    };
    
    let result = test.validate();
    assert!(result.is_err());
    
    let report = result.unwrap_err();
    let errors: Vec<_> = report.iter().collect();
    
    // We should have 12 errors (one for each field)
    assert_eq!(errors.len(), 12);
    
    // Check that all errors have custom codes
    for (path, error) in &errors {
        match error.code() {
            ErrorCode::Custom(code) => {
                match path.to_string().as_str() {
                    "email" => assert_eq!(code.as_str(), "CUSTOM_EMAIL"),
                    "url" => assert_eq!(code.as_str(), "CUSTOM_URL"),
                    "phone" => assert_eq!(code.as_str(), "CUSTOM_PHONE"),
                    "credit_card" => assert_eq!(code.as_str(), "CUSTOM_CC"),
                    "ip" => assert_eq!(code.as_str(), "CUSTOM_IP"),
                    "ascii_field" => assert_eq!(code.as_str(), "CUSTOM_ASCII"),
                    "alphanum_field" => assert_eq!(code.as_str(), "CUSTOM_ALPHANUM"),
                    "contains_field" => assert_eq!(code.as_str(), "CUSTOM_CONTAINS"),
                    "prefix_field" => assert_eq!(code.as_str(), "CUSTOM_PREFIX"),
                    "suffix_field" => assert_eq!(code.as_str(), "CUSTOM_SUFFIX"),
                    "pattern_field" => assert_eq!(code.as_str(), "CUSTOM_PATTERN"),
                    "length_field" => assert_eq!(code.as_str(), "CUSTOM_LENGTH"),
                    _ => panic!("Unexpected field: {}", path),
                }
            },
            _ => panic!("Expected custom error code for field: {}", path),
        }
    }
}

#[derive(Validate)]
struct StandardCodeTest {
    #[garde(email)]
    email: String,
    
    #[garde(length(3..=10))]
    length_field: String,
    
    #[garde(ascii)]
    ascii_field: String,
}

#[test]
fn test_standard_error_codes() {
    let test = StandardCodeTest {
        email: "invalid-email".to_string(),
        length_field: "ab".to_string(), // too short
        ascii_field: "非ASCII".to_string(),
    };
    
    let result = test.validate();
    assert!(result.is_err());
    
    let report = result.unwrap_err();
    let errors: Vec<_> = report.iter().collect();
    
    // Check that all errors have standard codes
    for (path, error) in &errors {
        match error.code() {
            ErrorCode::Standard(code) => {
                match path.to_string().as_str() {
                    "email" => assert_eq!(*code, StandardErrorCode::EmailInvalid),
                    "length_field" => assert_eq!(*code, StandardErrorCode::LengthMin),
                    "ascii_field" => assert_eq!(*code, StandardErrorCode::NotAscii),
                    _ => panic!("Unexpected field: {}", path),
                }
            },
            _ => panic!("Expected standard error code for field: {}", path),
        }
    }
}