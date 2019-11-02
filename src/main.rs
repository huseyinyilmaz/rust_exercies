use std::collections::HashMap;
use programming_exercises::stairs::count_ways;

fn main() {
    println!("count_ways: {}", count_ways(&20, &[1, 2], &mut HashMap::new()));
}
