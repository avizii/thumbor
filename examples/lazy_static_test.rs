#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref EXAMPLE: u8 = 1;

    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut map = HashMap::new();
        map.insert(0, "wu");
        map.insert(1, "de");
        map.insert(2, "jin");

        map
    };
    static ref COUNT: usize = HASHMAP.len();

    static ref NUMBER: u32 = times_two(21);
}

fn times_two(n: u32) -> u32 { n * 2 }

fn print_map_len(map: &HashMap<u32, &str>) {
    println!("the map len is {}.", map.len());
}

fn main() {
    // EXAMPLE's type is &u8, we need deref by * manually
    println!("the example is {}.", *EXAMPLE);
    // COUNT's type is &usize
    println!("the map has {} entries.", *COUNT);
    // NUMBER's type is &u32
    println!("an expensive calculation on a static result in: {}.", *NUMBER);

    // HASHMAP's type is &HashMap , get method's first param is &self, so here, we can use HASHMAP directly
    println!("the entry for '0' is '{}'", HASHMAP.get(&0).unwrap());

    // 使用 & 的方式来触发 Deref 仅引用类型的实参才会触发自动解引用
    print_map_len(&HASHMAP);
}