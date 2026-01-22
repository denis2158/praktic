fn main() {
    // FILL in the blank
    let b = true;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any type
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}
