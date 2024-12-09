fn main() {
    let data = [22, 12, 13, 17, 18];
    for i in data {
        let half = floored_half(i);
        println!("{}", half);
    }
}

fn floored_half(data: i32) -> i32{
    data / 2
}
