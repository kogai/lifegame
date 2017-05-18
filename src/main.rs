mod world;
use world::World;

fn main() {
    let game = World::new(10, 10);
    println!("{}", game.show());
}
