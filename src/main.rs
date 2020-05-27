fn main() {
    println!("Hello, world!");
    println!("result: {:?}", tas(1, 2));
}

fn tas(bef: i8, aft: i8) -> i8 {
    let result: i8 = bef + aft;
    return result;
}
