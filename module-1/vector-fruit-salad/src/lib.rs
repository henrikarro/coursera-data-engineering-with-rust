use rand::seq::{IndexedRandom, SliceRandom};
use std::fmt;

impl Fruit {
    pub fn new(name: &str) -> Self {
        Fruit(name.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Fruit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub struct Fruit(String);

// impl FromStr for Fruit {
//     type Err = ();

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(Fruit::new(s))
//     }
// }

// impl ToString for Fruit {
//     fn to_string(&self) -> String {
//         self.0.clone()
//     }
// }

pub struct FruitSalad {
    fruits: Vec<Fruit>,
    rng: rand::prelude::ThreadRng,
}

impl FruitSalad {
    pub fn new() -> Self {
        FruitSalad {
            fruits: Vec::new(),
            rng: rand::rng(),
        }
    }

    pub fn add_fruit(&mut self, fruit: Fruit) {
        self.fruits.push(fruit);
    }

    pub fn fruits(&self) -> &Vec<Fruit> {
        &self.fruits
    }

    pub fn mix(&mut self) {
        self.fruits.shuffle(&mut self.rng);
    }

    pub fn pick_random_fruit(&mut self) -> Option<&Fruit> {
        self.fruits.choose(&mut self.rng)
    }
}

impl fmt::Display for FruitSalad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fruit_names: Vec<String> = self.fruits.iter().map(|fruit| fruit.to_string()).collect();
        write!(f, "{}", fruit_names.join(", "))
    }
}