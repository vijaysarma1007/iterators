fn main() {
    let numbers = [10, 13, 23, 2, 8, 9, 6];

    let evens: Vec<i32> = numbers.into_iter().filter(|number| number % 2 == 0).collect();
    println!("{evens:?}");
}
