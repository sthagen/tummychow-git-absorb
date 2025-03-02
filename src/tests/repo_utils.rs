#[cfg(test)]
use std::path::{Path, PathBuf};
pub struct Context {
    pub repo: git2::Repository,
    pub dir: tempfile::TempDir,
}

impl Context {
    pub fn join(&self, p: &Path) -> PathBuf {
        self.dir.path().join(p)
    }
}

/// Prepare a fresh git repository with an initial commit and a file.
pub fn prepare_repo() -> (Context, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let repo = git2::Repository::init(dir.path()).unwrap();

    let path = PathBuf::from("test-file.txt");
    std::fs::write(
        dir.path().join(&path),
        br#"
line
line

more
lines
"#,
    )
    .unwrap();

    // make the borrow-checker happy by introducing a new scope
    {
        let tree = add(&repo, &path);
        let signature = repo
            .signature()
            .or_else(|_| git2::Signature::now("nobody", "nobody@example.com"))
            .unwrap();
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initial commit.",
            &tree,
            &[],
        )
        .unwrap();
    }

    (Context { repo, dir }, path)
}

/// Stage the changes made to `path`.
pub fn add<'r>(repo: &'r git2::Repository, path: &Path) -> git2::Tree<'r> {
    let mut index = repo.index().unwrap();
    index.add_path(&path).unwrap();
    index.write().unwrap();

    let tree_id = index.write_tree_to(&repo).unwrap();
    repo.find_tree(tree_id).unwrap()
}

/// Prepare an empty repo, and stage some changes.
pub fn prepare_and_stage() -> Context {
    let (ctx, file_path) = prepare_repo();

    // add some lines to our file
    let path = ctx.join(&file_path);
    let contents = std::fs::read_to_string(&path).unwrap();
    let modifications = format!("new_line1\n{contents}\nnew_line2");
    std::fs::write(&path, &modifications).unwrap();

    // stage it
    add(&ctx.repo, &file_path);

    ctx
}

pub fn become_new_author(repo: &git2::Repository) {
    let mut config = repo.config().unwrap();
    config.set_str("user.name", "nobody2").unwrap();
    config.set_str("user.email", "nobody2@example.com").unwrap();
}

/// Detach HEAD from the current branch.
pub fn detach_head(repo: &git2::Repository) {
    let head = repo.head().unwrap();
    let head_commit = head.peel_to_commit().unwrap();
    repo.set_head_detached(head_commit.id()).unwrap();
}

/// Delete the named branch from the repository.
pub fn delete_branch(repo: &git2::Repository, branch_name: &str) {
    let mut branch = repo
        .find_branch(branch_name, git2::BranchType::Local)
        .unwrap();
    branch.delete().unwrap();
}

/// Set the named repository config flag to true.
pub fn set_config_flag(repo: &git2::Repository, flag_name: &str) {
    repo.config().unwrap().set_str(flag_name, "true").unwrap();
}
