use std::collections::HashMap;

pub struct Monkey<'a> {
    pub items: Vec<usize>,
    pub thrown: usize,
    pub rule: Vec<&'a str>,
    pub div_by_test: usize,
    pub if_true: usize,
    pub if_false: usize
}

pub fn create_monkey(input: &str) -> Monkey {
    let lines: Vec<&str> = input.lines().collect();

    let items: Vec<usize> = lines[1].split(": ")
        .nth(1)
        .unwrap()
        .split(", ")
        .map(|s| usize::from_str_radix(s, 10).unwrap_or(0))
        .collect();

    let rule: Vec<&str> = lines[2].split_whitespace()
        .rev()
        .take(3) // The rule is reversed now, luckily + and * are commutative
        .collect();

    let div_by_test: usize = lines[3].split_whitespace()
        .map(|s| usize::from_str_radix(s, 10).unwrap_or(0))
        .last()
        .unwrap();

    let if_true: usize = lines[4].split_whitespace()
        .map(|s| usize::from_str_radix(s, 10).unwrap_or(0))
        .last()
        .unwrap();
        

    let if_false: usize = lines[5].split_whitespace()
        .map(|s| usize::from_str_radix(s, 10).unwrap_or(0))
        .last()
        .unwrap();

    Monkey {
        items,
        thrown: 0,
        rule,
        div_by_test,
        if_true,
        if_false,
    }
}

pub fn inspect_and_throw(monkeys: &mut Vec<Monkey>, index: usize, star: usize) {
    match monkeys[index].items.len() {
        0 => return,
        _ => {
            for _i in 0..monkeys[index].items.len() {
                // Inspect the item
                let mut local_item: usize = monkeys[index].items.pop().unwrap();
                local_item = operation(&monkeys[index].rule, local_item);
                
                if star == 1 {
                    // Momentary relief
                    local_item = local_item / 3;
                }
               
                // Test
                if local_item % monkeys[index].div_by_test == 0 {
                    let receiver: usize = monkeys[index].if_true;
                    monkeys[receiver].items.push(local_item);
                } else {
                    let receiver: usize = monkeys[index].if_false;
                    monkeys[receiver].items.push(local_item);
                }
                monkeys[index].thrown += 1;
            }
        }
    }
}

pub fn inspect_and_throw_cached(monkeys: &mut Vec<Monkey>, index: usize, _star: usize, cache: &mut HashMap<usize, usize>) {
    match monkeys[index].items.len() {
        0 => return,
        _ => {
            for _i in 0..monkeys[index].items.len() {
                // Inspect the item
                let mut local_item: usize = monkeys[index].items.pop().unwrap();
                local_item = operation(&monkeys[index].rule, local_item);
                
                // local_item = same_divisibility(local_item, cache, monkeys);
                local_item = same_properties(local_item, cache, monkeys);
                 
                // Test
                if local_item % monkeys[index].div_by_test == 0 {
                    let receiver: usize = monkeys[index].if_true;
                    monkeys[receiver].items.push(local_item);
                } else {
                    let receiver: usize = monkeys[index].if_false;
                    monkeys[receiver].items.push(local_item);
                }
                monkeys[index].thrown += 1;
            }
        }
    }
}

pub fn operation(rule: &Vec<&str>, item: usize) -> usize {
    if rule[0] == rule[2] { // old - old
        if rule[1] == "+" {
            return 2 * item;
        }
        if rule[1] == "*" {
            return item * item;
        }
    } else {
        let val: usize = usize::from_str_radix(rule[0], 10).unwrap();
        if rule[1] == "+" {
            return item + val;
        }
        if rule[1] == "*" {
            return item * val;
        }
    }
    0
}

// Does not work: for ex. 101 is not divisible by 2 or 3, so function would return 1, but 1 + 1 % 3 != 101 + 1 % 3
// Looks like the remainders also play a role
pub fn same_divisibility(input: usize, cache: &mut HashMap<usize, usize>, monkeys: &Vec<Monkey>) -> usize {
    if let Some(known_value) = cache.get(&input) {
        *known_value
    } else {
        let mut output: usize = 1;

        for i in 0..monkeys.len() {
            if input % monkeys[i].div_by_test == 0 {
                output *= monkeys[i].div_by_test;
            }
        }
    let _ = cache.insert(input, output);
    output
    }
}

// This one is far too slow
pub fn same_properties(input: usize, cache: &mut HashMap<usize, usize>, monkeys: &Vec<Monkey>) -> usize {
    // Store the remainders (where there is one) while building a number with the same divisibility
    if let Some(known_value) = cache.get(&input) {
        *known_value
    } else {
        let mut output: usize = 1;
        let mut remainders: HashMap<usize, usize> = HashMap::new();

        for i in 0..monkeys.len() {
            if input % monkeys[i].div_by_test == 0 {
                output *= monkeys[i].div_by_test;
            } else {
                remainders.insert(monkeys[i].div_by_test, input % monkeys[i].div_by_test);
            }
        }

        if output == 1 { // Don't waste time trying all possible numbers
            let _ = cache.insert(input, input);
            return input;
        }

        // We now have a number that divides the way we like it, but the remainders are off
        // Multiply it until it works out

        let mut found: bool = false;
        let mut attempt: usize = output;

        while !found {
            // Check if any remainders are off - in the worst case we should get the original input back, because it meets the conditions
            let mut correct: usize = 0;
            for key in remainders.keys() {
                if attempt % key == *remainders.get(key).unwrap() {
                    correct += 1;
                }
            }
            if remainders.len() == correct {
                found = true;
            } else {
                attempt += output;
            }
        }
        let _ = cache.insert(input, attempt);
        attempt
    }
}

