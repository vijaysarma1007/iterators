fn main() {
    let numbers = [10, 13, 23, 2, 8, 9, 6];
    let evens: Vec<i32> = numbers
        .iter()
        .filter(|number| *number % 2 == 0)
        .copied()
        .collect();
    println!("{evens:?}");

    let first_odd = numbers.into_iter().find(|number| number % 2 != 0);
    println!("First odd : {first_odd:?}");

    let nothing = numbers.into_iter().find(|number| *number > 100);
    println!("Nothing found: {nothing:?}");
}
