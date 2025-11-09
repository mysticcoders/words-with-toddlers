// Quick test to verify compound word parsing
use std::collections::HashSet;

fn main() {
    println!("Testing compound word detection...\n");
    
    // Test cases
    let test_cases = vec![
        ("dogcat", vec!["dog", "cat"]),
        ("ranintothepark", vec!["ran", "into", "the", "park"]),
        ("dogcatbird", vec!["dog", "cat", "bird"]),
        ("sunflower", vec!["sun", "flower"]),
        ("xyz123abc", vec![]),  // No valid words
    ];
    
    println!("Test Results:");
    println!("Input -> Expected Output");
    println!("-" .repeat(40));
    
    for (input, expected) in test_cases {
        println!("{} -> {:?}", input, expected);
        // Note: Actual testing would require importing Dictionary
        // This is just to show expected behavior
    }
    
    println!("\nHow it works:");
    println!("1. Type 'dogcat' and press Space");
    println!("   -> 'dog' and 'cat' appear at top");
    println!("2. Type 'ranintothepark' and press Space");
    println!("   -> 'ran', 'into', 'the', 'park' appear");
    println!("3. Spaces act as delimiters");
    println!("   'dog cat' with space -> processes each separately");
}