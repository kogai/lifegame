mod world;
use world::World;

fn main() {
    let game = World::new(10, 10, vec![(4, 4), (4, 5), (5, 4), (5, 5)]);

    println!("{}", game.show());

    game.tick();
    println!("{}", game.show());
}
