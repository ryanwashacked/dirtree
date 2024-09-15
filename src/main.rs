//! A tool for generating and updating directory structure representations in README files.
//!
//! This tool scans a specified directory, respects .gitignore rules, and generates
//! a tree-like representation of the directory structure. It then updates (or creates)
//! a README.md file with this structure.

#[cfg(test)]
mod tests;

use std::fs::{self, File};
use std::io::{Read, Write, Result as IoResult};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use ignore::gitignore::{GitignoreBuilder, Gitignore};
use clap::{Parser, Command};

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
}

/// Builds a Gitignore instance for the given root directory
///
/// # Arguments
///
/// * `root` - The root directory path
///
/// # Returns
///
/// A Result containing the Gitignore instance or an IO error
fn build_gitignore(root: &Path) -> IoResult<Gitignore> {
    let mut builder = GitignoreBuilder::new(root);
    let gitignore_path = root.join(".gitignore");
    println!("Debug: Searching for .gitignore at {:?}", gitignore_path);
    if gitignore_path.exists() {
        println!("Debug: .gitignore file found at {:?}", gitignore_path);
        builder.add(&gitignore_path);
        let mut content = String::new();
        File::open(&gitignore_path)?.read_to_string(&mut content)?;
        println!("Debug: .gitignore content:\n{}", content);
    } else {
        println!("Debug: No .gitignore file found at {:?}", gitignore_path);
    }
    let gitignore = builder.build().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    println!("Debug: Gitignore built successfully");
    Ok(gitignore)
}

/// Checks if a path should be ignored based on gitignore rules
///
/// # Arguments
///
/// * `path` - The path to check
/// * `gitignore` - The Gitignore instance to use for checking
///
/// # Returns
///
/// A boolean indicating whether the path should be ignored
fn should_ignore(path: &Path, gitignore: &Gitignore) -> bool {
    let is_ignored = gitignore.matched_path_or_any_parents(path, path.is_dir()).is_ignore();
    println!("Debug: Checking path {:?}, is_dir: {}, is_ignored: {}", path, path.is_dir(), is_ignored);
    is_ignored
}
/// Determines the icon to use for a file or directory
///
/// # Arguments
///
/// * `name` - The name of the file or directory
/// * `is_dir` - Whether the item is a directory
///
/// # Returns
///
/// A string slice containing the appropriate icon
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
///
/// # Arguments
///
/// * `path` - The root path to start the tree generation from
/// * `gitignore` - The Gitignore instance to use for filtering
/// * `max_depth` - The maximum depth to traverse (0 means no limit)
///
/// # Returns
///
/// A Result containing the generated tree as a String or an IO error
fn generate_tree(path: &Path, gitignore: &Gitignore, max_depth: usize) -> IoResult<String> {
    let mut tree = String::new();
    let mut stack = vec![(path.to_path_buf(), 0)];
    let mut is_last = HashMap::new();

    while let Some((current_path, depth)) = stack.pop() {
        let is_dir = current_path.is_dir();
        let name = current_path.file_name().unwrap().to_string_lossy();

        if should_ignore(&current_path, gitignore) {
            continue;
        }

        let prefix = (0..depth).map(|d| {
            if is_last.get(&d) == Some(&true) {
                "   "
            } else {
                "   " // Changed from "│  " to "   "
            }
        }).collect::<String>();

        let icon = get_icon(&name, is_dir);

        tree.push_str(&format!("{}{} {}\n", prefix, icon, name));

        if is_dir && (max_depth == 0 || depth < max_depth) {
            let mut entries: Vec<_> = fs::read_dir(&current_path)?
                .filter_map(Result::ok)
                .filter(|e| !should_ignore(&e.path(), gitignore))
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
///
/// # Arguments
///
/// * `tree` - The generated directory structure as a string
/// * `path` - The path to the directory containing the README.md file
///
/// # Returns
///
/// A Result indicating success or an IO error
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
    println!("
    ░█▀▄░▀█▀░█▀▄░▀█▀░█▀▄░█▀▀░█▀▀
    ░█░█░░█░░█▀▄░░█░░█▀▄░█▀▀░█▀▀
    ░▀▀░░▀▀▀░▀░▀░░▀░░▀░▀░▀▀▀░▀▀▀

    ░█▀▀░█▀▀░█▀█░█▀▀░█▀▄░█▀█░▀█▀░█▀▀░█▀▄
    ░█░█░█▀▀░█░█░█▀▀░█▀▄░█▀█░░█░░█▀▀░█░█
    ░▀▀▀░▀▀▀░▀░▀░▀▀▀░▀░▀░▀░▀░░▀░░▀▀▀░▀▀░

    Your directory tree has been successfully generated!
    ");
}

fn main() -> IoResult<()> {
    let opts = Opts::parse();

    let target_dir = opts.target_dir.unwrap_or_else(|| std::env::current_dir().unwrap());

    if !target_dir.exists() {
        eprintln!("Error: The specified directory does not exist.");
        std::process::exit(1);
    }

    let gitignore = build_gitignore(&target_dir)?;
    let tree = generate_tree(&target_dir, &gitignore, opts.depth)?;
    update_readme(&tree, &target_dir)?;

    display_success_message();

    println!("README.md has been updated with the directory structure in {:?}", target_dir);
    Ok(())
}