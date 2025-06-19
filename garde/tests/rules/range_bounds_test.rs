use garde::Validate;

#[derive(Debug, garde::Validate)]
struct Test {
    #[garde(range(min = 10, max = 100))]
    field: u64,
}

#[test]
fn test_existing_range_snapshot() {
    let cases = [
        Test { field: 5 },    // too low
        Test { field: 150 },  // too high
    ];
    
    let mut snapshot = String::new();
    for case in cases {
        if let Err(report) = case.validate() {
            snapshot.push_str(&format!("{case:#?}\n"));
            snapshot.push_str(&format!("{report}\n\n"));
        }
    }
    
    insta::assert_snapshot!(snapshot);
}