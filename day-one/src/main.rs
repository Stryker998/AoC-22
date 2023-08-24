use std::{fs, cell::RefCell};
fn main() {
    let mut inventories: Vec<RefCell<Vec<i32>>> = vec![];


    let string = fs::read_to_string("input.txt").unwrap_or_else(|err| {
        panic!("{}", err);
    });
    let mut counter = 0;
    inventories.push(
        RefCell::new(vec![])
    );
    string.lines().for_each(|line| {
        if line.is_empty() {
            counter += 1;
            inventories.push(RefCell::new(vec![]));
        } else {
            inventories.get(counter).unwrap().borrow_mut().push(line.parse().unwrap());
        };
    });

    let mut sums: Vec<i32> = vec![];
    inventories.iter().for_each(|inventory| {
        let current_sum: i32 = inventory.borrow().iter().sum();
        sums.push(current_sum);
    });

    sums.sort();
    sums.reverse();

    let max_sum = sums.first().unwrap();
    println!("{}", max_sum);

    let sum_of_three: i32 = sums.get(0..3).unwrap().iter().sum();

    println!("{}", sum_of_three);
}
