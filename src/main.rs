mod world;

use std::thread::sleep;
use std::time::Duration;
use world::World;

fn main() {
    let mut game = World::new(20, 20, vec![(4, 4), (4, 5), (5, 4), (5, 5), (6, 4), (6, 5)]);

    println!("{}", game.show());
    println!("------");

    loop {
        game.tick();
        println!("{}", game.show());
        println!("------");
        sleep(Duration::from_secs(1));
    }
}
