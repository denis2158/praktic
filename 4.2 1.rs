fn main() {
    let c1 = 'a';
    assert_eq!(c1.len_utf8(), 1); // 'a' у UTF-8 займає 1 байт

    let c2 = '中';
    assert_eq!(c2.len_utf8(), 3); // '中' у UTF-8 займає 3 байти

    println!("Success!");
}