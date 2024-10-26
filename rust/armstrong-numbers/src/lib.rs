pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    let n = num_string.len();
    num_string.chars()
        .filter_map(|c| c.to_digit(10).map(|digit| digit.pow(n as u32))).sum::<u32>() == num
}
