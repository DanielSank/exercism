use robot_simulator::*;

fn main() {
    println!("{:?}", (Direction::North as i8 - 1) % 4);
}
