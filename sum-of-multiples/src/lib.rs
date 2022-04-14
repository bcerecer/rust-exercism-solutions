pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // Go through each number from 1 to limit and check if such number is a multiple of each factor
    // If it is, add it to the sum and after going through them, return sum
    (1..limit)
        .filter(|num| -> bool {
            factors
                .iter()
                .filter(|fac| -> bool {**fac > 0})
                .any(|fac| num % fac == 0)
        })
        .sum()
}
