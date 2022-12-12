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

pub fn inspect_and_throw(monkeys: &mut Vec<Monkey>, index: usize, star: usize, magic_number: &usize) {
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

                if local_item > *magic_number {
                    local_item = local_item % *magic_number;
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
