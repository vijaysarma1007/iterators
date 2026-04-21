use std::collections::HashSet;

#[derive(Debug)]
struct PlayList {
    songs: Vec<String>,
    users: HashSet<String>,
}

impl FromIterator<(String, String)> for PlayList {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let mut songs = Vec::new();
        let mut users = HashSet::new();
        for (song, user) in iter {
            songs.push(song);
            users.insert(user);
        }
        Self { songs, users }
    }
}

fn main() {
    let fifty_numbers = 1..=50;
    let results = Vec::from_iter(fifty_numbers.clone());
    println!("{results:?}");

    let results = fifty_numbers.clone().collect::<Vec<i32>>();
    println!("{results:?}");

    let unique_set: HashSet<_> = HashSet::from_iter(fifty_numbers.clone());
    println!("{unique_set:?}");

    let chars = ['H', 'e', 'l', 'l', 'o'];
    let greetings = String::from_iter(chars);
    println!("{}", greetings);

    let songs = [
        (String::from("I run go on"), String::from("Bob")),
        (String::from("A Rust of Wind"), String::from("Bob")),
        (String::from("A Rustworthy Man"), String::from("Sheila")),
    ];

    let playlist = PlayList::from_iter(songs);
    println!("{playlist:?}")
}
