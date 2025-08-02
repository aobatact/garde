use super::util;

#[derive(Debug, garde::Validate)]
struct TestStruct {
    #[garde(rename("custom_name"), length(5..))]
    field: String,
    #[garde(range(10..))]
    normal_field: i32,
}

#[test]
fn rename_field_in_error_path() {
    let invalid = TestStruct {
        field: "abc".to_string(), // too short
        normal_field: 5, // too small
    };
    
    let result = garde::Validate::validate(&invalid);
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    let error_string = format!("{:?}", error);
    
    // Check that the renamed field appears with its custom name in the error
    assert!(error_string.contains("custom_name"), "Error should contain renamed field 'custom_name', got: {}", error_string);
    
    // Check that the normal field appears with its original name
    assert!(error_string.contains("normal_field"), "Error should contain original field 'normal_field', got: {}", error_string);
}

#[test] 
fn rename_field_valid() {
    let valid = TestStruct {
        field: "valid_string".to_string(),
        normal_field: 15,
    };
    
    util::check_ok(&[valid], &());
}