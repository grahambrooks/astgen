use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    let git_sha = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .expect("Failed to execute git command")
        .stdout;

    let git_sha = String::from_utf8_lossy(&git_sha).trim().to_string();

    let out_dir = env::var("OUT_DIR").unwrap();
    let version_file = format!("{}/version.txt", out_dir);

    std::fs::write(&version_file, git_sha).expect("Failed to write version file");

    println!("cargo:rustc-env=VERSION_FILE={}", version_file);

    // Extract tree-sitter dependency versions from Cargo.toml
    generate_version_file(&out_dir);

    // Make the build script rerun if Cargo.toml changes
    println!("cargo:rerun-if-changed=Cargo.toml");
}

fn generate_version_file(out_dir: &str) {
    // Read Cargo.toml content
    let cargo_toml = std::fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");

    // Parse the TOML content (simple approach using string search)
    let mut versions = Vec::new();

    extract_version(&cargo_toml, "tree-sitter-rust", &mut versions);
    extract_version(&cargo_toml, "tree-sitter-java", &mut versions);
    extract_version(&cargo_toml, "tree-sitter-c-sharp", &mut versions);
    extract_version(&cargo_toml, "tree-sitter-go", &mut versions);
    extract_version(&cargo_toml, "tree-sitter-python", &mut versions);
    extract_version(&cargo_toml, "tree-sitter-typescript", &mut versions);
    extract_version(&cargo_toml, "tree-sitter-javascript", &mut versions);
    extract_version(&cargo_toml, "tree-sitter-ruby", &mut versions);

    // Generate Rust code
    let mut code = String::from("// Auto-generated file - DO NOT EDIT\n\n");

    for (name, version) in versions {
        let const_name = name.replace('-', "_").to_uppercase();
        code.push_str(&format!(
            "pub const {}_VERSION: &str = \"{}\";\n",
            const_name, version
        ));
    }

    // Write the generated code to a file
    let dest_path = Path::new(out_dir).join("versions_gen.rs");
    let mut file = File::create(dest_path).expect("Failed to create versions_gen.rs");
    file.write_all(code.as_bytes())
        .expect("Failed to write versions_gen.rs");
}

fn extract_version(cargo_toml: &str, package_name: &str, versions: &mut Vec<(String, String)>) {
    let search_str = format!("{} = ", package_name);

    if let Some(pos) = cargo_toml.find(&search_str) {
        let start_pos = pos + search_str.len();
        let version_str = &cargo_toml[start_pos..];

        // Handle both "x.y.z" and { version = "x.y.z", ... } formats
        if let Some(stripped) = version_str.strip_prefix('"') {
            if let Some(end_pos) = stripped.find('"') {
                versions.push((package_name.to_string(), stripped[..end_pos].to_string()));
            }
        } else if version_str.starts_with('{') {
            let version_search = "version = \"";
            if let Some(version_pos) = version_str.find(version_search) {
                let ver_start = version_pos + version_search.len();
                let ver_str = &version_str[ver_start..];
                if let Some(end_pos) = ver_str.find('"') {
                    versions.push((package_name.to_string(), ver_str[..end_pos].to_string()));
                }
            }
        }
    }
}
