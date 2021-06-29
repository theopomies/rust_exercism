pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    let num_len = num_string.len() as u32;

    num_string
        .chars()
        .fold(0, |sum, curr| sum + curr.to_digit(10).unwrap().pow(num_len))
        == num
}
