fn main() {
    let data = [22, 12, 13, 17, 18];
    let mut result = [0; 5];
    for i in 0..5 {
        result[i] = floored_half(data[i]);
    }
    println!("{:?}", result);
}

fn floored_half(data: i32) -> i32 {
    data / 2
}
