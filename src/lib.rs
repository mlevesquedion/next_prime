/// Returns the ceiling of the square root of an unsigned integer in a
/// timely manner, using binary search.  
/// Time complexity: O(log(n))
fn usqrt(n: u64) -> u64 {
    let (mut low, mut high) = (1, n);
    let mut mid = (low + high) / 2;
    while low < high {
        mid = (low + high) / 2;
        let square = mid * mid;
        if square == n {
            return mid;
        } else if square > n {
            high = mid - 1
        } else {
            low = mid + 1
        }
    }
    if mid * mid == n {
        mid
    } else {
        high
    }
}

#[cfg(test)]
mod usqrt_tests {
    use super::usqrt;

    #[test]
    fn small_perfect_squares() {
        let inputs = (1..11).collect::<Vec<u64>>();
        let squares = inputs.iter().map(|x| x * x).collect::<Vec<u64>>();
        let outputs = squares.into_iter().map(usqrt).collect::<Vec<u64>>();
        assert_eq!(inputs, outputs);
    }

    #[test]
    fn large_perfect_squares() {
        let inputs = (1000..10000).step_by(37).collect::<Vec<u64>>();
        let squares = inputs.iter().map(|x| x * x).collect::<Vec<u64>>();
        let outputs = squares.into_iter().map(usqrt).collect::<Vec<u64>>();
        assert_eq!(inputs, outputs);
    }

    #[test]
    fn edge_case_zero() {
        assert_eq!(usqrt(0), 0);
    }

    #[test]
    fn edge_case_one() {
        assert_eq!(usqrt(1), 1);
    }

    #[test]
    fn rounds_up_when_not_a_perfect_square() {
        assert_eq!(usqrt(2), 2);
    }

    #[test]
    fn large_not_perfect_square() {
        let x = 12345;
        assert_eq!(usqrt(x * x + 1), x + 1)
    }
}

/// Determines whether a number is prime or not.  
/// Time complexity: O(sqrt(n))
fn is_prime(n: u64) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n <= 1 {
        return false;
    }
    let lower = 3;
    let upper = usqrt(n);
    (lower..(upper + 1))
        .step_by(2)
        .all(|maybe_divisor| n % maybe_divisor != 0)
}

#[cfg(test)]
mod is_prime_tests {
    use super::is_prime;

    #[test]
    fn small_primes() {
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23];
        assert_eq!(
            primes
                .clone()
                .into_iter()
                .map(is_prime)
                .collect::<Vec<bool>>(),
            vec![true; primes.len()]
        );
    }
}

/// Finds the next prime number >= n.  
/// Time complexity: *expected* O(sqrt(n))
/// # Examples
/// ```
/// use next_prime::next_prime;
/// assert_eq!(next_prime(2), 2);
/// assert_eq!(next_prime(4), 5);
/// ```
pub fn next_prime(mut n: u64) -> u64 {
    if n <= 2 {
        return 2;
    }
    if n % 2 == 0 {
        n += 1;
    }
    while !is_prime(n) {
        n += 2;
    }
    n
}

#[cfg(test)]
mod next_prime_tests {
    use super::next_prime;

    #[test]
    fn edge_case_two() {
        assert_eq!(next_prime(2), 2);
    }

    #[test]
    fn finds_small_primes() {
        let primes = vec![5, 7, 11, 13, 17, 19, 23, 29];
        assert_eq!(
            primes,
            primes
                .iter()
                .map(|x| next_prime(x - 1))
                .collect::<Vec<u64>>()
        );
    }

    #[test]
    fn returns_argument_when_it_is_already_prime() {
        assert_eq!(next_prime(101), 101);
    }

    #[test]
    fn finds_a_very_large_prime() {
        assert_eq!(next_prime(472_888_178), 472_888_217)
    }
}
