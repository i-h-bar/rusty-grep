use std::cmp;

pub fn distance(str1: &str, str2: &str) -> usize {
    let len_str_1 = str1.chars().count() + 1;
    let len_str_2 = str2.chars().count() + 1;

    let mut mat: Vec<Vec<usize>> = vec![vec![0; len_str_2]; len_str_1];

    for i in 1..len_str_1 {
        mat[i][0] = i;
    }
    for j in 1..len_str_2 {
        mat[0][j] = j;
    }

    for (i, chr1) in str1.chars().enumerate() {
        for (j, chr2) in str2.chars().enumerate() {
            if chr1 == chr2 {
                mat[i + 1][j + 1] = mat[i][j]
            } else {
                mat[i + 1][j + 1] = cmp::min(mat[i][j + 1], cmp::min(mat[i + 1][j], mat[i][j])) + 1;
            }
        }
    }

    mat[len_str_1 - 1][len_str_2 - 1]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn equal_strings() {
        assert_eq!(distance("Hello World!", "Hello World!"), 0)
    }

    #[test]
    fn not_equal() {
        assert_eq!(distance("HelloWorld", "Hello World!"), 2)
    }

    #[test]
    fn emoji_dist_equal() {
        assert_eq!(distance("🍆💩", "🍆💩"), 0)
    }

    #[test]
    fn emoji_dist_not_equal() {
        assert_eq!(distance("🍆💩", "💩🍆"), 2)
    }
}
