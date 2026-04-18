fn main() {
    let seafood = "Oyster 🦪";

    for byte in seafood.bytes() {
        print!("{byte}/");
    }

    for character in seafood.chars() {
        print!("{character}/");
    }

    println!("{}", seafood.bytes().len());
    println!("{}", seafood.chars().count());
}
