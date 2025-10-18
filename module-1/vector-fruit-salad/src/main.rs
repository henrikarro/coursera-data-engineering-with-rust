/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/

use clap::Parser;
use vector_fruit_salad::{Fruit, FruitSalad};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[clap(short, long, default_value = "false", help = "Add extra fruits to the salad")]
    add_extra_fruits: bool,
    #[clap(value_delimiter = ',', num_args = 0.., help="Additional fruits to add to the salad")]
    fruits: Vec<String>,
}

static DEFAULT_FRUIT_NAMES: &[&str] = &["Orange", "Fig", "Pomegranate", "Cherry", "Apple", "Pear", "Peach"];

static EXTRA_FRUIT_NAMES: &[&str] = &["Mango", "Grapes", "Banana"];

fn main() {
    let args = Args::parse();

    let mut fruit_salad = make_basic_fruit_salad();

    if args.add_extra_fruits {
        add_extra_fruits_to_salad(&mut fruit_salad);
    }

    for fruit_name in args.fruits {
        fruit_salad.add_fruit(Fruit::new(&fruit_name));
    }

    fruit_salad.mix();

    println!("Fruit Salad: {}", fruit_salad);
    println!("Random fruit from salad: {}", fruit_salad.pick_random_fruit().unwrap());
}

fn make_basic_fruit_salad() -> FruitSalad {
    let mut fruit_salad = FruitSalad::new();
    for &fruit_name in DEFAULT_FRUIT_NAMES {
        fruit_salad.add_fruit(Fruit::new(fruit_name));
    }
    fruit_salad
}

fn add_extra_fruits_to_salad(fruit_salad: &mut FruitSalad) {
    for &fruit_name in EXTRA_FRUIT_NAMES {
        fruit_salad.add_fruit(Fruit::new(fruit_name));
    }
}
