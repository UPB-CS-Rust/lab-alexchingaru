fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    // TODO
    let largest_value = input.iter().max();
    let smallest_value = input.iter().min();
    
    if let (Some(largest_value), Some(smallest_value)) = (largest_value, smallest_value){
        println!("{} is largest and {} is smallest", largest_value, smallest_value);
    }
}