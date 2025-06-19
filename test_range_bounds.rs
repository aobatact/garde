use garde::Validate;

#[derive(Debug, garde::Validate)]
struct RangeBounds {
    #[garde(range(10..=100))]
    inclusive_range: u64,
}

fn main() {
    let valid = RangeBounds { inclusive_range: 50 };
    println!("{:?}", valid.validate(&()));
    
    let invalid = RangeBounds { inclusive_range: 5 };
    println!("{:?}", invalid.validate(&()));
}