fn main() {
    let numbers = vec![4, 8, 15, 16, 23, 42];
    let sqaures = numbers
        .iter()
        .map(|number: &i32| number.pow(2))
        .collect::<Vec<i32>>();
    println!("{sqaures:?}");
}
