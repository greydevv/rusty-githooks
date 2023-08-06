use std::process::Command;

pub struct BranchInfo {
    pub name: String,
}

impl BranchInfo {
    pub fn new() -> Self {
        BranchInfo {
            name: BranchInfo::get_branch_name(),
        }
    }

    fn get_branch_name() -> String {
        let c = Command::new("git").arg("branch").arg("--show-current").output().unwrap().stdout;
        return String::from_utf8(c).unwrap()
    }
}
