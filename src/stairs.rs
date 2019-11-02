/*
There exists a staircase with N steps, and you can climb up either 1 or 2 steps at a time. Given N, write a function that returns the number of unique ways you can climb the staircase. The order of the steps matters.

For example, if N is 4, then there are 5 unique ways:

1, 1, 1, 1
2, 1, 1
1, 2, 1
1, 1, 2
2, 2
What if, instead of being able to climb 1 or 2 steps at a time, you could climb any number from a set of positive integers X? For example, if X = {1, 3, 5}, you could climb 1, 3, or 5 steps at a time.
*/

use std::collections::HashMap;

pub fn count_ways(number_of_stairs: &i32, options: &[i32], cache : &mut HashMap<i32, i32>) -> i32 {
    match cache.get(number_of_stairs) {
        Some(result) => {
            println!("found result in cache: count_ways({}) => {}", number_of_stairs, result);
            *result
        }
        None => {
            println!("Could not find the value in cache: {}", number_of_stairs);
            let result = match number_of_stairs {
                _ if number_of_stairs < &0 => 0,
                0 | 1 => 1,
                _ => {
                    options.iter().map(|step| count_ways(&(number_of_stairs - step), options, cache)).sum()
                }

            };
            println!("Save result in cache: count_ways({}) => {}", number_of_stairs, result);
            cache.insert(*number_of_stairs, result);
            result
        }
    }
}


#[test]
fn test_5() {
    let mut cache = &mut HashMap::new();
    assert_eq!(count_ways(&4, &[1, 2], &mut cache), 5);
}

#[test]
fn test_20() {
    let mut cache = &mut HashMap::new();
    assert_eq!(count_ways(&20, &[1, 2], &mut cache), 10946);
}

#[test]
fn test_20_with_custom_steps() {
    let mut cache = &mut HashMap::new();
    assert_eq!(count_ways(&50, &[3, 5, 7], &mut cache), 49286);
}
