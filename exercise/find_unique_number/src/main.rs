use std::collections::HashMap;

fn find_unique_number(numbers: &[i32]) -> Option<i32> {
    let mut count_map: HashMap<i32, usize> = HashMap::new();

    // Count the occurrences of each number
    for &num in numbers {
        *count_map.entry(num).or_insert(0) += 1;
    }

    // Find the number with a count of 1
    for (&num, &count) in &count_map {
        if count == 1 {
            return Some(num);
        }
    }

    None // Return None if no unique number is found
}

fn main() {
    let numbers = vec![1, 1, 2, 2, 3, 3, 4, 5, 5, 5, 9];
    if let Some(unique_number) = find_unique_number(&numbers) {
        println!("Unique number: {}", unique_number);
    } else {
        println!("No unique number found");
    }
}
