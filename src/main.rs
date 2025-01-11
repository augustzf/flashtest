use git2::{Repository, DiffOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the current Git repository
    let repo = Repository::open(".")?;

    // Get the current branch
    let head_ref = repo.head()?;
    let head_name = head_ref.shorthand().ok_or("Failed to get branch name")?;
    println!("Current branch: {}", head_name);

    // Find the remote tracking branch (e.g., origin/main)
    let remote_branch = format!("refs/remotes/origin/{}", head_name);
    let remote_ref = repo.find_reference(&remote_branch)?;
    let remote_commit = remote_ref.peel_to_commit()?;

    // Get the latest local commit
    let head_commit = head_ref.peel_to_commit()?;

    // Generate a diff between the local and remote commits
    let mut diff_options = DiffOptions::new();
    let diff = repo.diff_tree_to_tree(
        Some(&head_commit.tree()?),
        Some(&remote_commit.tree()?),
        Some(&mut diff_options),
    )?;


    // Collect and print all modified files
    let mut modified_files = Vec::new();
    println!("diff count: {}", diff.deltas().count());
    diff.foreach(
        &mut |delta, _| {
            println!("delta: {:?} foo", &delta);
            if let Some(path) = delta.new_file().path() {
                modified_files.push(path.to_owned());
            }
            true
        },
        None,
        None,
        None,
    )?;

    println!("Modified files since last push:");
    for file in modified_files {
        println!("{}", file.display());
    }

    Ok(())
}
