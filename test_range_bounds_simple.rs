//! Simple test file to test range bounds functionality

use garde::Validate;

#[derive(Debug, garde::Validate)]
struct TestBounds {
    #[garde(range(10..100))]
    value: u32,
}

fn main() {
    // Test valid case
    let valid = TestBounds { value: 50 };
    match valid.validate(&()) {
        Ok(_) => println!("✓ Valid case passed"),
        Err(e) => println!("✗ Valid case failed: {}", e),
    }

    // Test invalid case - too low
    let invalid_low = TestBounds { value: 5 };
    match invalid_low.validate(&()) {
        Ok(_) => println!("✗ Invalid low case should have failed"),
        Err(e) => println!("✓ Invalid low case correctly failed: {}", e),
    }

    // Test invalid case - too high  
    let invalid_high = TestBounds { value: 150 };
    match invalid_high.validate(&()) {
        Ok(_) => println!("✗ Invalid high case should have failed"),
        Err(e) => println!("✓ Invalid high case correctly failed: {}", e),
    }
}