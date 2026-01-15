// Make println! work
fn main() {
    let _f: bool = false;

    let t = true;
    if t { // замінили !t на t
        println!("Success!");
    }
}