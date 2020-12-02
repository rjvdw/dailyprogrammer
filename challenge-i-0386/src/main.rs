mod base_sequence_generator;
mod secondary_sequence_generator;

use std::collections::HashMap;
use secondary_sequence_generator::SecondarySequenceGenerator;

fn main() {
    let mut cache: HashMap<u64, u64> = HashMap::new();

    println!("{}", solve(&mut cache, 0));
    println!("{}", solve(&mut cache, 1));
    println!("{}", solve(&mut cache, 2));
    println!("{}", solve(&mut cache, 3));
    println!("{}", solve(&mut cache, 4));
    println!("{}", solve(&mut cache, 5));
    println!("{}", solve(&mut cache, 6));
    println!("{}", solve(&mut cache, 66));
    println!("{}", solve(&mut cache, 333));
    println!("{}", solve(&mut cache, 410)); // largest value before overflow
}

fn solve(mut cache: &mut HashMap<u64, u64>, n: u64) -> u64 {
    if n == 0 {
        1
    } else if cache.contains_key(&n) {
        *cache.get(&n).unwrap()
    } else {
        let numbers = reverse_iter(SecondarySequenceGenerator::new(n));
        let mut sign: u8 = ((numbers.len() - 1) % 4) as u8;
        let mut values_to_add: u64 = 0;
        let mut values_to_subtract: u64 = 0;

        for i in numbers {
            let next = solve(&mut cache, n - i);
            if sign < 2 {
                values_to_add += next;
            } else {
                values_to_subtract += next;
            }
            if values_to_add > values_to_subtract {
                values_to_add -= values_to_subtract;
                values_to_subtract = 0;
            }
            sign = (sign + 3) % 4;
        }

        let result = values_to_add - values_to_subtract;
        cache.insert(n, result);
        result
    }
}

fn reverse_iter(iterator: SecondarySequenceGenerator) -> Vec<u64> {
    let mut numbers = Vec::new();
    numbers.extend(iterator);
    numbers.reverse();
    numbers
}
