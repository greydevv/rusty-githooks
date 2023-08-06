use rusty_gh::BranchInfo;

fn main() {
    let b = BranchInfo::new();
    println!("{}", b.name)
}
