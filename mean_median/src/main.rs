use regex::Regex;
use std::{collections::HashMap, io};

fn main() {
    println!("Please provide a list of numbers ");
    println!("ex: 1 3 4 590 5 4 4 ");

    loop {
        let mut numbers = String::new();

        io::stdin()
            .read_line(&mut numbers)
            .expect("Error reading numbers");

        let expression = Regex::new(r"\s+").unwrap();

        let arr: Vec<i32> = expression
            .split(&numbers.trim())
            .map(|x| x.parse::<i32>().expect("Invalid input for number"))
            .collect();

        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut mean = 0.0;
        let len = arr.len() as f64;

        for element in arr {
            mean += element as f64;
            let el = map.entry(element).or_default();
            *el += 1
        }

        mean /= len;
        let mut max_key = 0;
        let mut max_val = 0;

        for (key, val) in map {
            if val > max_val {
                max_val = val;
                max_key = key
            }
        }

        println!("Mean is {mean:.2}");
        println!("Mode is {max_key}");
    }
}
