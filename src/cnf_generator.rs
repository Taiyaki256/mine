/// ちょうどK個のリテラルが真となる制約を生成
pub fn exactly_k(variables: &[i32], k: usize, next_var: &mut i32) -> Vec<Vec<i32>> {
    let mut clauses = at_most_k(variables, k, next_var);
    clauses.extend(at_least_k(variables, k));
    clauses
}

/// 少なくともK個のリテラルが真となる制約を生成
fn at_least_k(variables: &[i32], k: usize) -> Vec<Vec<i32>> {
    let n = variables.len();
    if k == 0 {
        return Vec::new();
    }
    if n < k {
        panic!("k must be less than or equal to the number of variables");
    }

    let comb_size = n - k + 1;
    variables
        .iter()
        .combinations(comb_size)
        .map(|comb| comb.into_iter().copied().collect())
        .collect()
}
