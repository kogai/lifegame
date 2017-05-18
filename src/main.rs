mod world;

use std::thread::sleep;
use std::time::Duration;
use world::World;

fn main() {
    let mut game = World::new(
        10, 10,
        vec![
            (7, 1),
            (5, 2), (7, 2), (8, 2),
            (5, 3), (7, 3),
            (5, 4),
            (3, 5),
            (1, 6), (3, 6),
        ]
    );

    println!("{}", game.show());
    println!("------");

    loop {
        game.tick();
        println!("{}", game.show());
        println!("------");
        sleep(Duration::from_millis(500));
    }
}
