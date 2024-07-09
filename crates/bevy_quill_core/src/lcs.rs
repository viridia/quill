/// Longest common substring
pub fn lcs<T1, T2, F>(arr1: &[T1], arr2: &[T2], comparator: F) -> (usize, usize, usize)
where
    F: Fn(&T1, &T2) -> bool,
{
    let mut longest = 0;
    let mut arr1_offset = 0;
    let mut arr2_offset = 0;
    let mut dp = vec![vec![0; arr2.len() + 1]; arr1.len() + 1];

    for i in 1..=arr1.len() {
        for j in 1..=arr2.len() {
            if comparator(&arr1[i - 1], &arr2[j - 1]) {
                dp[i][j] = dp[i - 1][j - 1] + 1;
                if dp[i][j] > longest {
                    longest = dp[i][j];
                    arr1_offset = i - longest;
                    arr2_offset = j - longest;
                }
            }
        }
    }

    (arr1_offset, arr2_offset, longest)
}

#[cfg(test)]
mod tests {
    use super::lcs;

    #[test]
    fn test_empty() {
        let a: Vec<i32> = vec![];
        let b: Vec<i32> = vec![];

        let (start_a, start_b, length) = lcs(&a, &b, |x, y| x == y);
        assert_eq!(start_a, 0);
        assert_eq!(start_b, 0);
        assert_eq!(length, 0);
    }

    #[test]
    fn test_empty_left() {
        let a: Vec<i32> = vec![];
        let b: Vec<i32> = vec![0, 1];

        let (start_a, start_b, length) = lcs(&a, &b, |x, y| x == y);
        assert_eq!(start_a, 0);
        assert_eq!(start_b, 0);
        assert_eq!(length, 0);
    }

    #[test]
    fn test_empty_right() {
        let a: Vec<i32> = vec![];
        let b: Vec<i32> = vec![0, 1];

        let (start_a, start_b, length) = lcs(&a, &b, |x, y| x == y);
        assert_eq!(start_a, 0);
        assert_eq!(start_b, 0);
        assert_eq!(length, 0);
    }

    #[test]
    fn test_different() {
        let a: Vec<i32> = vec![1, 2];
        let b: Vec<i32> = vec![3, 4];

        let (start_a, start_b, length) = lcs(&a, &b, |x, y| x == y);
        assert_eq!(start_a, 0);
        assert_eq!(start_b, 0);
        assert_eq!(length, 0);
    }

    #[test]
    fn test_same() {
        let a: Vec<i32> = vec![1, 2, 3];
        let b: Vec<i32> = vec![1, 2, 3];

        let (start_a, start_b, length) = lcs(&a, &b, |x, y| x == y);
        assert_eq!(start_a, 0);
        assert_eq!(start_b, 0);
        assert_eq!(length, 3);
    }

    #[test]
    fn test_insert_start() {
        let a: Vec<i32> = vec![0, 1, 2, 3];
        let b: Vec<i32> = vec![1, 2, 3];

        let (start_a, start_b, length) = lcs(&a, &b, |x, y| x == y);
        assert_eq!(start_a, 1);
        assert_eq!(start_b, 0);
        assert_eq!(length, 3);
    }

    #[test]
    fn test_insert_end() {
        let a: Vec<i32> = vec![1, 2, 3, 4];
        let b: Vec<i32> = vec![1, 2, 3];

        let (start_a, start_b, length) = lcs(&a, &b, |x, y| x == y);
        assert_eq!(start_a, 0);
        assert_eq!(start_b, 0);
        assert_eq!(length, 3);
    }

    #[test]
    fn test_diff() {
        let a = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        let b = vec![2, 7, 1, 8, 2, 8, 1, 8, 2, 8, 4, 5, 9, 0];

        let (start_a, start_b, length) = lcs(&a, &b, |x, y| x == y);
        assert_eq!(start_a, 4);
        assert_eq!(start_b, 11);
        assert_eq!(length, 2);
    }
}
