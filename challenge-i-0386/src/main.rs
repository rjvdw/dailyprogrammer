use std::collections::HashMap;

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
}

fn solve(mut cache: &mut HashMap<u64, u64>, n: u64) -> u64 {
    if n == 0 {
        1
    } else if cache.contains_key(&n) {
        *cache.get(&n).unwrap()
    } else {
        let numbers = reverse_iter(SecondarySequenceGenerator::new(n));
        let mut sign: u8 = ((numbers.len() - 1) % 4) as u8;
        let mut pos: u64 = 0;
        let mut neg: u64 = 0;

        for i in numbers {
            let next = solve(&mut cache, n - i);
            if sign < 2 {
                pos += next;
            } else {
                neg += next;
            }
            sign = (sign + 3) % 4;
        }

        let result = pos - neg;
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

struct SecondarySequenceGenerator {
    i: u64,
    s: BaseSequenceGenerator,
    u: u64,
}

impl SecondarySequenceGenerator {
    fn new(u: u64) -> SecondarySequenceGenerator {
        SecondarySequenceGenerator { i: 1, s: BaseSequenceGenerator::new(), u }
    }
}

impl Iterator for SecondarySequenceGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i <= self.u {
            let i = self.i;
            self.i += self.s.next().unwrap();
            Some(i)
        } else {
            None
        }
    }
}

struct BaseSequenceGenerator {
    s1: u64,
    s2: u64,
    b: bool,
}

impl BaseSequenceGenerator {
    fn new() -> BaseSequenceGenerator {
        BaseSequenceGenerator { s1: 0, s2: 1, b: false }
    }
}

impl Iterator for BaseSequenceGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.b = !self.b;
        if self.b {
            self.s1 += 1;
            Some(self.s1)
        } else {
            self.s2 += 2;
            Some(self.s2)
        }
    }
}
