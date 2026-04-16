use std::collections::HashMap;

fn main() {
    let numbers = vec![4, 8, 15, 16];
    let mut my_iterator = numbers.into_iter();

    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator);

    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator);

    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator);

    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator);

    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator);

    let mut my_hasmap = HashMap::new();
    my_hasmap.insert("CBS", 2);
    my_hasmap.insert("CBS2", 3);
    let hashmap_iter = my_hasmap.into_iter();

    for hash in hashmap_iter {
        println!("{:?}", hash);
    }

    let numbers = vec![4, 8, 15, 16];
    let my_iterator = numbers.iter();

    for number in my_iterator {
        println!("{number}");
    }

    println!("{numbers:?}");

    let cities = vec![String::from("Phoenix"), String::from("Dallas")];

    for city in cities.iter() {
        println!("{city}");
    }

    println!("{cities:?}");
}
