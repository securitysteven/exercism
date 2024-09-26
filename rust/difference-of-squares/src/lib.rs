pub fn square_of_sum(n: u32) -> u32 {
    // let sum = n * (n + 1) / 2;  // Sum of first n natural numbers
    // sum * sum  // Square of the sum using multiplication
    (1..=n).sum::<u32>().pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|x| x.pow(2)).sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
