#[derive(Debug)]
struct GasStation {
    snack_count: u32,
    manager: String,
    employee_count: u32,
}

fn main() {
    let mut points = [3, 8, 1, 11, 5];
    println!("is sorted: {}", points.is_sorted());

    points.sort();
    println!("Sorted Array: {:?}", points);

    points.reverse();
    println!("decending order: {:?}", points);

    let mut exercises = ["squat", "bench", "deadlift"];
    exercises.sort();
    println!("Sorted string: {:?}", exercises);

    let mobile = GasStation {
        snack_count: 100,
        manager: String::from("Meg Mobile"),
        employee_count: 12,
    };

    let exxon = GasStation {
        snack_count: 130,
        manager: String::from("Eric Exxon"),
        employee_count: 5,
    };

    let shell = GasStation {
        snack_count: 50,
        manager: String::from("Shane Shell"),
        employee_count: 3,
    };

    let mut stops = [mobile, exxon, shell];
    stops.sort_by_key(|station| station.snack_count);
    println!("Ordered stops: {:#?}", stops);
}
