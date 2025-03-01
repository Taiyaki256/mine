pub mod cnf_generator;

use cnf_generator::exactly_k;
use itertools::Itertools;
use splr::{cdb::ClauseDBIF, *};

fn main() {
    let mut rules: Vec<Vec<i32>> = Vec::new();

    let config = splr::Config {
        splr_interface: true,
        quiet_mode: false,
        ..Default::default()
    };
    // let mut next_var = 10; // 既存の変数が1-4を使用している場合
    // rules.extend(exactly_k(&vec![2, 5, 4], 1, &mut next_var));
    // rules.extend(exactly_k(&vec![2, 5, 6], 1, &mut next_var));
    // rules.extend(exactly_k(&vec![1, 2, 5, 8, 7], 1, &mut next_var));
    // rules.extend(exactly_k(&vec![1, 2, 3, 4, 6, 7, 8, 9], 2, &mut next_var));
    // rules.extend(exactly_k(&vec![2, 3, 5, 8, 9], 2, &mut next_var));
    // rules.extend(exactly_k(&vec![7, 4, 5, 6, 9], 1, &mut next_var));
    // rules.extend(exactly_k(&vec![4, 5, 8], 0, &mut next_var));

    let mut next_var = 5; // 既存の変数が1-4を使用している場合
    let variables = vec![1, 2, 3, 4];
    let cnf = exactly_k(&variables, 1, &mut next_var);
    rules.extend(cnf);
    let mut solver = Solver::try_from((config, rules.as_ref())).expect("panic");
    for model in solver.iter() {
        println!("found!");
        println!("{:?}", model);
        // 見つかった解を除外する制約を追加
        let mut clause = Vec::new();
        for lit in model.iter() {
            clause.push(-lit);
        }
        println!("{:?}", clause);
        let err = solver.add_clause(clause);
        println!("{:?}", err);
    }
    // println!("Total solutions: {}", models.len());
}
