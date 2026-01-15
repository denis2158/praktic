fn main() {
    let c1 = "中";
    print_char(c1.chars().next().unwrap());
} 

fn print_char(c: char) {
    println!("{}", c);
}