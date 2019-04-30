use crate::install::Requirements;
use git2::{Reference, Repository};
use std::env;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, git2::Error>;

pub fn branch(name: &str) -> Result<Requirements> {
    let repo = open()?;
    let old = repo.head()?;

    let branch = repo
        .find_branch(name, git2::BranchType::Local)
        .or(repo.branch(name, &old.peel_to_commit()?, false))?;
    let reference = branch.get();
    repo.checkout_tree(&reference.peel(git2::ObjectType::Tree)?, None)?;
    repo.set_head(reference.name().expect("Invalid branch name."))?;
    let new = repo.head()?;

    pull(&repo, &format!("refs/remotes/origin/{}", name))?;

    generate_requirements(&repo, old, new)
}

pub fn tag(name: &str) -> Result<Requirements> {
    let name = name.trim_start_matches("v");

    let repo = open()?;
    let old = repo.head()?;
    let refname = &format!("refs/tags/{}", name);

    let tag = repo.find_tag(repo.refname_to_id(refname)?)?;
    repo.checkout_tree(&tag.peel()?, None)?;
    repo.set_head(refname)?;
    let new = repo.head()?;

    generate_requirements(&repo, old, new)
}

pub fn default() -> Result<Requirements> {
    let repo = open()?;
    let old = repo.head()?;

    if !old.is_branch() {
        return Err(git2::Error::from_str("HEAD must point to a valid branch."));
    }

    pull(
        &repo,
        &old.name()
            .expect("Failed to get current branch.")
            .replace("heads", "remotes/origin"),
    )?;
    let new = repo.head()?;
    generate_requirements(&repo, old, new)
}

fn open() -> Result<Repository> {
    let path = env::current_dir().unwrap_or(PathBuf::from("."));
    Repository::open(path)
}

fn pull(repo: &Repository, name: &str) -> Result<()> {
    let mut remote = repo.find_remote("origin")?;
    remote.fetch(&[name], None, None)?;

    let oid = repo.refname_to_id(name)?;
    let object = repo.find_object(oid, None)?;
    repo.reset(&object, git2::ResetType::Hard, None)
}

fn generate_requirements(
    repo: &Repository,
    old: Reference,
    new: Reference,
) -> Result<Requirements> {
    let diff = repo.diff_tree_to_tree(
        old.peel_to_tree().as_ref().ok(),
        new.peel_to_tree().as_ref().ok(),
        None,
    )?;
    Ok(Requirements::from_deltas(diff.deltas()))
}
