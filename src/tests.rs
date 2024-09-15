use super::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::create_dir_all;
    use tempfile::TempDir;

    #[test]
    fn test_get_icon() {
        assert_eq!(get_icon("folder", true), "📁");
        assert_eq!(get_icon("script.sh", false), "📜");
        assert_eq!(get_icon("config.yml", false), "🔧");
        assert_eq!(get_icon("key.pem", false), "🔑");
        assert_eq!(get_icon("readme.md", false), "📝");
        assert_eq!(get_icon("notes.txt", false), "📄");
        assert_eq!(get_icon("query.sql", false), "📊");
        assert_eq!(get_icon("unknown.xyz", false), "📄");
    }

    #[test]
    fn test_should_ignore() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        println!("Debug: Test root directory: {:?}", root);

        // Create a .gitignore file
        let gitignore_path = root.join(".gitignore");
        let mut gitignore_file = File::create(&gitignore_path).unwrap();
        writeln!(gitignore_file, "*.log\nnode_modules/\nbuild/").unwrap();

        println!("Debug: Created .gitignore at {:?}", gitignore_path);
        println!("Debug: .gitignore content:");
        let mut content = String::new();
        File::open(&gitignore_path).unwrap().read_to_string(&mut content).unwrap();
        println!("{}", content);

        let gitignore = build_gitignore(root).unwrap();

        // Test files
        assert!(should_ignore(&root.join("error.log"), &gitignore), "error.log should be ignored");
        assert!(!should_ignore(&root.join("main.rs"), &gitignore), "main.rs should not be ignored");

        // Test directories
        let node_modules_path = root.join("node_modules");
        println!("Debug: Checking node_modules at {:?}", node_modules_path);
        create_dir_all(&node_modules_path).unwrap(); // Create the node_modules directory
        assert!(should_ignore(&node_modules_path, &gitignore), "node_modules should be ignored");

        let build_path = root.join("build");
        create_dir_all(&build_path).unwrap(); // Create the build directory
        assert!(should_ignore(&build_path, &gitignore), "build should be ignored");

        let src_path = root.join("src");
        create_dir_all(&src_path).unwrap(); // Create the src directory
        assert!(!should_ignore(&src_path, &gitignore), "src should not be ignored");

        // Test nested paths
        File::create(node_modules_path.join("package.json")).unwrap();
        assert!(should_ignore(&node_modules_path.join("package.json"), &gitignore), "node_modules/package.json should be ignored");

        File::create(build_path.join("output.txt")).unwrap();
        assert!(should_ignore(&build_path.join("output.txt"), &gitignore), "build/output.txt should be ignored");

        File::create(src_path.join("main.rs")).unwrap();
        assert!(!should_ignore(&src_path.join("main.rs"), &gitignore), "src/main.rs should not be ignored");
    }

    #[test]
    fn test_generate_tree() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create a simple directory structure
        create_dir_all(root.join("src")).unwrap();
        create_dir_all(root.join("tests")).unwrap();
        File::create(root.join("src/main.rs")).unwrap();
        File::create(root.join("Cargo.toml")).unwrap();
        File::create(root.join("README.md")).unwrap();

        let gitignore = build_gitignore(root).unwrap();
        let tree = generate_tree(root, &gitignore, 0).unwrap();

        let expected_tree = format!(
            "📁 {}\n   📁 src\n      📄 main.rs\n   📁 tests\n   📄 Cargo.toml\n   📝 README.md\n",
            root.file_name().unwrap().to_string_lossy()
        );

        assert_eq!(tree, expected_tree);
    }

    #[test]
    fn test_update_readme() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create an initial README.md
        let initial_content = "# My Project\n\nSome description.\n\n## Features\n\n- Feature 1\n- Feature 2\n";
        fs::write(root.join("README.md"), initial_content).unwrap();

        let tree = "📁 project\n   📄 file1.txt\n   📄 file2.txt\n";
        update_readme(tree, root).unwrap();

        let updated_content = fs::read_to_string(root.join("README.md")).unwrap();
        let expected_content = format!("{}## Directory Structure\n\n```\n{}\n```\n", initial_content, tree);

        assert_eq!(updated_content, expected_content);
    }
}