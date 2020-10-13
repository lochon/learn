// mod guess;
mod common_programming_concepts;

// use guess::game::guess_game;
use common_programming_concepts::variables::*;
use common_programming_concepts::data_types::*;
fn main() {
    // guess_game()
    var();
    println!("heartbeat is {}", HEARTBEAT_127);
    shadowing();

    statically_typed();
    scalar();
}
