use itertools::Itertools;
use splr::{SatSolverIF, SolveIF, Solver};

/// 最大K個制約を生成（補助変数使用版）
/// - variables: 制約を適用するリテラルのスライス
/// - k: 許容される最大真リテラル数
/// - next_var: 補助変数生成用カウンター（可変参照）
pub fn at_most_k(variables: &[i32], k: usize, next_var: &mut i32) -> Vec<Vec<i32>> {
    let n = variables.len();
    if n <= k {
        return Vec::new();
    }
    if k == 0 {
        return variables.iter().map(|&x| vec![-x]).collect();
    }

    let mut clauses = Vec::new();
    let mut counter_vars: Vec<Vec<i32>> = Vec::with_capacity(n);

    // 補助変数の初期化
    for i in 0..n {
        let mut row = Vec::with_capacity(k + 1);
        for j in 0..=k {
            row.push(*next_var);
            *next_var += 1;
        }
        counter_vars.push(row);
    }

    // シーケンシャルカウンターエンコーディング
    for i in 0..n {
        for j in 0..=k {
            if i > 0 {
                // 前のステップの伝搬
                clauses.push(vec![-counter_vars[i - 1][j], counter_vars[i][j]]);
            }
            if j > 0 && i > 0 {
                // カウンターの増加を制限
                clauses.push(vec![
                    -variables[i],
                    -counter_vars[i - 1][j - 1],
                    counter_vars[i][j],
                ]);
            }
        }
        // 現在のリテラルとカウンターの関係
        clauses.push(vec![-variables[i], counter_vars[i][0]]);
    }

    // 最終制約（k+1個目のカウンターを禁止）
    if n > 0 {
        clauses.push(vec![-counter_vars[n - 1][k]]);
    }

    clauses
}

/// ちょうどK個のリテラルが真となる制約を生成
pub fn exactly_k(variables: &[i32], k: usize, next_var: &mut i32) -> Vec<Vec<i32>> {
    let mut clauses = at_most_k(variables, k, next_var);
    clauses.extend(at_least_k(variables, k));
    clauses
}

/// 少なくともK個のリテラルが真となる制約を生成
pub fn at_least_k(variables: &[i32], k: usize) -> Vec<Vec<i32>> {
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

/// 含意制約 (a → b)
/// 変数aが真なら変数bも真
pub fn implies(a: i32, b: i32) -> Vec<Vec<i32>> {
    vec![vec![-a, b]]
}

/// 双方向含意制約 (a ↔ b)
/// 変数aとbは同値
pub fn equivalent(a: i32, b: i32) -> Vec<Vec<i32>> {
    vec![vec![-a, b], vec![-b, a]]
}

pub fn mineguess(solver: &mut Solver, cell_num: usize) -> Vec<i32> {
    let mut first_model: Vec<i32> = Vec::new();

    // 最初の解を取得
    for ans in solver.iter().take(1) {
        first_model = ans.clone();
    }
    // 最初の解をベースに確定値を調査
    let mut sure_model = vec![0; cell_num];

    for (i, lit) in first_model.iter().enumerate() {
        if i >= cell_num {
            break;
        }
        // 反対の値を仮定して解を探索
        if solver.add_clause(vec![-*lit]).is_ok() {
            sure_model[i] = 0;
        } else {
            sure_model[i] = *lit;
        }
    }
    sure_model
}
