pub fn count_bits(input: &[&str], index: usize) -> isize {

    let ones = input.iter().fold(0usize, |n, line| {
        if line.chars().nth(index) == Some('1') {
            n + 1
        } else {
            n
        }
    });

    if ones > input.len() - ones {
        1
    } else if ones == input.len() - ones {
        0
    } else {
        -1
    }
}