fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    // TODO
    let largest = input.iter().max().unwrap();
    let smallest = input.iter().min().unwrap();

    println!("{} is largest and {} is smallest", largest, smallest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emppty_array_should_return_none() {
        let input = [];
        let largest = input.iter().max().unwrap();
        let smallest = input.iter().min().unwrap();
        assert_eq!(largest, None);
        assert_eq!(smallest, None);
    }
}