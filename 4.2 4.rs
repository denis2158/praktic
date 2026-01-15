fn main() {
    let f = false;        // змінили на false
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}

