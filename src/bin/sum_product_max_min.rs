fn main() {
    let numbers = vec![4, 8, 15, 16, 23, 42];
    let total: i32 = numbers.iter().sum();
    println!("Sum: {total}");

    let product: i32 = numbers.iter().product();
    println!("Product: {product}");

    let max = numbers.iter().max().unwrap();
    println!("Max: {max}");

    let min = numbers.iter().min().unwrap();
    println!("Min: {min}");

    let count = numbers.iter().count();
    println!("Count: {count}");

    let numbers = vec![4.6, 8.8, 0.0 / 0.0, f64::NAN];
    let total = numbers
        .iter()
        .filter(|number| !number.is_nan())
        .copied()
        .fold(0.0, |total, current| total + current);
}
