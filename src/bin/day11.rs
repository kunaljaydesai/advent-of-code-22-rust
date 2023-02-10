use core::num;
use std::collections::VecDeque;

type Worry = i32;

fn TestFn<const Divisor: i32, const TrueMonkey: usize, const FalseMonkey: usize>(
    worry: Worry,
) -> usize {
    if worry % Divisor == 0 {
        return TrueMonkey;
    }
    return FalseMonkey;
}

struct Monkey {
    items: VecDeque<Worry>,
    operation: fn(Worry) -> i32,
    test: fn(Worry) -> usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut monkeys: [Monkey; 8] = [
        Monkey {
            items: VecDeque::from([78, 53, 89, 51, 52, 59, 58, 85]),
            operation: |old| return old * 3,
            test: |worry| return TestFn::<5, 2, 7>(worry),
        },
        Monkey {
            items: VecDeque::from([64]),
            operation: |old| return old + 7,
            test: |worry| {
                return TestFn::<2, 3, 6>(worry);
            },
        },
        Monkey {
            items: VecDeque::from([71, 93, 65, 82]),
            operation: |old| return old + 5,
            test: |worry| {
                return TestFn::<13, 5, 4>(worry);
            },
        },
        Monkey {
            items: VecDeque::from([67, 73, 95, 75, 56, 74]),
            operation: |old| return old + 8,
            test: |worry| {
                return TestFn::<19, 6, 0>(worry);
            },
        },
        Monkey {
            items: VecDeque::from([85, 91, 90]),
            operation: |old| return old + 4,
            test: |worry| {
                return TestFn::<11, 3, 1>(worry);
            },
        },
        Monkey {
            items: VecDeque::from([67, 96, 69, 55, 70, 83, 62]),
            operation: |old| return old * 2,
            test: |worry| {
                return TestFn::<3, 4, 1>(worry);
            },
        },
        Monkey {
            items: VecDeque::from([53, 86, 98, 70, 64]),
            operation: |old| return old + 6,
            test: |worry| {
                return TestFn::<7, 7, 0>(worry);
            },
        },
        Monkey {
            items: VecDeque::from([88, 64]),
            operation: |old| return old * old,
            test: |worry| {
                return TestFn::<17, 2, 5>(worry);
            },
        },
    ];
    let mut num_inspections = [0; 8];
    for round in 0..20 {
        let mut monkey_idx = 0;
        loop {
            if monkey_idx >= 8 {
                break;
            }
            let mut moved_worry: [Vec<Worry>; 8] = Default::default();
            {
                let monkey = &mut monkeys[monkey_idx];
                while let Some(mut item_worry) = monkey.items.pop_front() {
                    num_inspections[monkey_idx] += 1;
                    // update fn
                    item_worry = (monkey.operation)(item_worry);

                    // relief
                    item_worry /= 3;

                    // test
                    let to_monkey_idx = (monkey.test)(item_worry);
                    moved_worry[to_monkey_idx].push(item_worry);
                }
                monkey_idx += 1;
            }
            for to_monkey_idx in 0..moved_worry.len() {
                let to_monkey = &mut monkeys[to_monkey_idx];
                to_monkey.items.extend(&moved_worry[to_monkey_idx]);
            }
        }
    }
    for monkey_idx in 0..8 {
        let num_inspection = num_inspections[monkey_idx];
        println!("Monkey {} : {}", monkey_idx, num_inspection);
    }
    Ok(())
}
