mod world;
use world::World;

fn main() {
    let mut game = World::new(10, 10, vec![(4, 4), (4, 5), (5, 4), (5, 5)]);

    println!("{}", game.show());
    println!("------");

    game.tick();
    println!("{}", game.show());
    println!("------");
}
