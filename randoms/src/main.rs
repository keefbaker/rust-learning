use rand::seq::SliceRandom;

fn main() {
    let vs = vec!["sausage", "bacon", "eggs"];
    println!("{:?}", vs.choose(&mut rand::thread_rng()).unwrap());
}