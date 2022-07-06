pub struct Commits {
    r1: i32,
    r2: i32,
}

impl Commits {
    pub fn new(r1: i32, r2: i32) -> Commits {
        Commits { r1: r1, r2: r2 }
    }
}

pub struct User {
    name : String,
    commits: Commits
}
