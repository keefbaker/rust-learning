use rand::seq::SliceRandom;

fn main() {
    let vs = vec!["sausage", "bacon", "eggs"];
    println!("For Breakfast I want {}", vs.choose(&mut rand::thread_rng()).unwrap());
}