use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Debug)]
enum Team {
    A,
    B,
}

fn main() {
    let mut points = HashMap::new();

    points.insert(Team::A, 10);
    points.insert(Team::B, 15);

    for (team, point) in points.iter() {
        println!("{team:?}: {point}");
    }

    let point_a = points.entry(Team::A).or_insert(0);
    *point_a += 1;

    let point_a = points.get(&Team::A);
    if let Some(p) = point_a {
        println!("A: {p}")
    }
}
