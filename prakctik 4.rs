fn main() {
    println!("{}, world", define_x());
}

fn define_x() -> &'static str {
    let x = "hello";
    x
}
