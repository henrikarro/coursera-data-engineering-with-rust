use std::collections::HashMap;

fn main() {
    let mut fruits = vec!("apple", "banana", "cherry", "apple");
    print_fruits("Initial fruits", &fruits);
    fruits.push("fig");
    print_fruits("After push", &fruits);
    let popped_fruit = fruits.pop().unwrap();
    println!("Popped fruit: {}", popped_fruit);
    print_fruits("Reamining fruits", &fruits);
    fruits = remove_fruit("banana", fruits);
    print_fruits("After removing 'banana'", &fruits);
    fruits = sort_fruits(fruits);
    print_fruits("AFter sorting", &fruits);
    let counts = count_fruits(fruits);
    for (fruit, count) in counts.iter() {
        println!("{} occurs {} times", fruit, count);
    }
}

fn print_fruits(heading: &str, fruits: &Vec<&'static str>) {
    println!("{}: {}", heading, fruits.join(", "));
}

fn remove_fruit<'a>(fruit: &'a str, mut fruits: Vec<&'a str>) -> Vec<&'a str> {
    if let Some(index) = fruits.iter().position(|f| *f == fruit) {
        fruits.remove(index);
    }
    fruits
}

fn sort_fruits(mut fruits: Vec<&str>) -> Vec<&str> {
    fruits.sort();
    fruits
}

fn count_fruits(fruits: Vec<&str>) -> HashMap<&str, usize> {
    let mut counts = HashMap::new();
    for fruit in fruits.iter() {
        *(counts.entry(*fruit).or_insert(0)) += 1;
    }
    counts
}
