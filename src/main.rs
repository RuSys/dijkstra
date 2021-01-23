mod core;

fn main() {
    let mut n = core::Dj::new(4);
    n.distance_set(0, 1, 2);
    n.distance_set(0, 2, 3);
    n.distance_set(0, 3, 9);
    n.distance_set(1, 3, 1);
    n.distance_set(2, 3, 4);
    let p = n.run(0, 3);
    print!("{:?}", p);

    println!("End Dijkstra")
}
