pub fn square_of_sum(n: u32) -> u32 {
    let mut squares_of_sum = 0;
    for num in 0..=n {
        squares_of_sum += num;
    }
    squares_of_sum * squares_of_sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum_of_squares = 0;
    for num in 0..=n {
        sum_of_squares += num*num;
    }
    sum_of_squares
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
