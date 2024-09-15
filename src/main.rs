#[cfg(test)]
mod tests;

use std::fs::{self, File};
use std::io::{Read, Write, Result as IoResult};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use ignore::gitignore::{GitignoreBuilder, Gitignore};
use clap::Parser;
use colored::*;

/// Command line options for the directory structure generator
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    /// Number of subdirectory levels to expand (0 means no limit)
    #[arg(short, long, default_value = "0")]
    depth: usize,

    /// Target directory to analyze
    #[arg(value_name = "DIR")]
    target_dir: Option<PathBuf>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

/// Builds a Gitignore instance for the given root directory
fn build_gitignore(root: &Path, verbose: bool) -> IoResult<Gitignore> {
    let mut builder = GitignoreBuilder::new(root);
    let gitignore_path = root.join(".gitignore");
    if verbose {
        println!("{}", format!("Searching for .gitignore at {:?}", gitignore_path).cyan());
    }
    if gitignore_path.exists() {
        if verbose {
            println!("{}", format!(".gitignore file found at {:?}", gitignore_path).green());
        }
        builder.add(&gitignore_path);
        let mut content = String::new();
        File::open(&gitignore_path)?.read_to_string(&mut content)?;
        if verbose {
            println!("{}", ".gitignore content:".yellow());
            println!("{}", content);
        }
    } else if verbose {
        println!("{}", format!("No .gitignore file found at {:?}", gitignore_path).yellow());
    }
    let gitignore = builder.build().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    if verbose {
        println!("{}", "Gitignore built successfully".green());
    }
    Ok(gitignore)
}

/// Checks if a path should be ignored based on gitignore rules
fn should_ignore(path: &Path, gitignore: &Gitignore, verbose: bool) -> bool {
    let is_ignored = gitignore.matched_path_or_any_parents(path, path.is_dir()).is_ignore();
    if verbose {
        println!("{}", format!("Checking path {:?}, is_dir: {}, is_ignored: {}", path, path.is_dir(), is_ignored).cyan());
    }
    is_ignored
}

/// Determines the icon to use for a file or directory
fn get_icon(name: &str, is_dir: bool) -> &'static str {
    if is_dir {
        "📁"
    } else {
        match name.rsplit('.').next() {
            Some("yml") | Some("yaml") => "🔧",
            Some("sh") => "📜",
            Some("pem") => "🔑",
            Some("md") => "📝",
            Some("txt") => "📄",
            Some("sql") => "📊",
            _ => "📄"
        }
    }
}

/// Generates a tree-like representation of the directory structure
fn generate_tree(path: &Path, gitignore: &Gitignore, max_depth: usize, verbose: bool) -> IoResult<String> {
    let mut tree = String::new();
    let mut stack = vec![(path.to_path_buf(), 0)];
    let mut is_last = HashMap::new();

    while let Some((current_path, depth)) = stack.pop() {
        let is_dir = current_path.is_dir();
        let name = current_path.file_name().unwrap().to_string_lossy();

        if should_ignore(&current_path, gitignore, verbose) {
            continue;
        }

        let prefix = (0..depth).map(|d| {
            if is_last.get(&d) == Some(&true) {
                "   "
            } else {
                "   "
            }
        }).collect::<String>();

        let icon = get_icon(&name, is_dir);

        tree.push_str(&format!("{}{} {}\n", prefix, icon, name));

        if is_dir && (max_depth == 0 || depth < max_depth) {
            let mut entries: Vec<_> = fs::read_dir(&current_path)?
                .filter_map(Result::ok)
                .filter(|e| !should_ignore(&e.path(), gitignore, verbose))
                .collect();

            entries.sort_by(|a, b| {
                let a_is_dir = a.file_type().map(|t| t.is_dir()).unwrap_or(false);
                let b_is_dir = b.file_type().map(|t| t.is_dir()).unwrap_or(false);
                match (a_is_dir, b_is_dir) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.file_name().cmp(&b.file_name()),
                }
            });

            for (i, entry) in entries.into_iter().rev().enumerate() {
                stack.push((entry.path(), depth + 1));
                is_last.insert(depth + 1, i == 0);
            }
        }
    }

    Ok(tree)
}

/// Updates the README.md file with the generated directory structure
fn update_readme(tree: &str, path: &Path) -> IoResult<()> {
    let readme_path = path.join("README.md");
    let mut content = String::new();

    if readme_path.exists() {
        let mut file = File::open(&readme_path)?;
        file.read_to_string(&mut content)?;
    }

    let dir_structure_start = content.find("## Directory Structure");
    let dir_structure_end = content[dir_structure_start.unwrap_or(content.len())..]
        .find("\n## ")
        .map(|pos| pos + dir_structure_start.unwrap_or(0))
        .unwrap_or(content.len());

    let new_content = if dir_structure_start.is_some() {
        format!(
            "{}## Directory Structure\n\n```\n{}\n```\n{}",
            &content[..dir_structure_start.unwrap()],
            tree,
            &content[dir_structure_end..]
        )
    } else {
        format!(
            "{}## Directory Structure\n\n```\n{}\n```\n",
            content, tree
        )
    };

    let mut file = File::create(&readme_path)?;
    file.write_all(new_content.as_bytes())?;

    Ok(())
}

fn display_success_message() {
    println!("{}", "
    ░█▀▄░▀█▀░█▀▄░▀█▀░█▀▄░█▀▀░█▀▀
    ░█░█░░█░░█▀▄░░█░░█▀▄░█▀▀░█▀▀
    ░▀▀░░▀▀▀░▀░▀░░▀░░▀░▀░▀▀▀░▀▀▀

    ░█▀▀░█▀▀░█▀█░█▀▀░█▀▄░█▀█░▀█▀░█▀▀░█▀▄
    ░█░█░█▀▀░█░█░█▀▀░█▀▄░█▀█░░█░░█▀▀░█░█
    ░▀▀▀░▀▀▀░▀░▀░▀▀▀░▀░▀░▀░▀░░▀░░▀▀▀░▀▀░

    Your directory tree has been successfully generated!
    ".green());
}

fn main() -> IoResult<()> {
    let opts = Opts::parse();

    let target_dir = opts.target_dir.unwrap_or_else(|| std::env::current_dir().unwrap());

    if !target_dir.exists() {
        eprintln!("{}", "Error: The specified directory does not exist.".red());
        std::process::exit(1);
    }

    let gitignore = build_gitignore(&target_dir, opts.verbose)?;
    let tree = generate_tree(&target_dir, &gitignore, opts.depth, opts.verbose)?;
    update_readme(&tree, &target_dir)?;

    display_success_message();

    println!("{}", format!("README.md has been updated with the directory structure in {:?}", target_dir).green());
    Ok(())
}