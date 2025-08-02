use garde::{Validate, Error, ErrorCode, StandardErrorCode};

#[derive(Validate)]
struct TestErrorCodes {
    // カスタムエラーコード付きのemail
    #[garde(email)]
    #[garde(code("CUSTOM_EMAIL_ERROR"))]
    email: String,
    
    // 標準エラーコード（codeなし）
    #[garde(email)]
    standard_email: String,
    
    // カスタムエラーコード付きのlength
    #[garde(length(3..=10))]
    #[garde(code("CUSTOM_LENGTH_ERROR"))]
    username: String,
    
    // 標準エラーコード（codeなし）
    #[garde(length(3..=10))]
    standard_length: String,
}

#[test]
fn test_custom_error_codes() {
    let test = TestErrorCodes {
        email: "invalid-email".to_string(),
        standard_email: "invalid-email".to_string(),
        username: "ab".to_string(), // too short
        standard_length: "ab".to_string(), // too short
    };
    
    let result = test.validate();
    assert!(result.is_err());
    
    let report = result.unwrap_err();
    let errors: Vec<_> = report.iter().collect();
    
    // Check that we have 4 errors
    assert_eq!(errors.len(), 4);
    
    // Find errors for each field
    let email_error = &errors.iter().find(|(path, _)| path.to_string() == "email").unwrap().1;
    let standard_email_error = &errors.iter().find(|(path, _)| path.to_string() == "standard_email").unwrap().1;
    let username_error = &errors.iter().find(|(path, _)| path.to_string() == "username").unwrap().1;
    let standard_length_error = &errors.iter().find(|(path, _)| path.to_string() == "standard_length").unwrap().1;
    
    // Check error codes
    match email_error.code() {
        ErrorCode::Custom(code) => assert_eq!(code.as_str(), "CUSTOM_EMAIL_ERROR"),
        _ => panic!("Expected custom error code for email field"),
    }
    
    match standard_email_error.code() {
        ErrorCode::Standard(StandardErrorCode::EmailInvalid) => {},
        _ => panic!("Expected standard EmailInvalid error code for standard_email field"),
    }
    
    match username_error.code() {
        ErrorCode::Custom(code) => assert_eq!(code.as_str(), "CUSTOM_LENGTH_ERROR"),
        _ => panic!("Expected custom error code for username field"),
    }
    
    match standard_length_error.code() {
        ErrorCode::Standard(StandardErrorCode::LengthMin) => {},
        _ => panic!("Expected standard LengthMin error code for standard_length field"),
    }
}