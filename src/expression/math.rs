/// Compute n!
pub fn factorial(n: usize) -> usize {
    fn factorial_impl(val: usize, n: usize) -> usize {
        if n == 1 {
            val
        } else {
            factorial_impl(val * (n - 1), n - 1)
        }
    }

    factorial_impl(n, n)
}
