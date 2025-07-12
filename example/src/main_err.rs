// SPDX-License-Identifier: MIT
fn fs_err() -> localtrace::Result<()> {
    std::fs::read_to_string(String::from("non_existent_file.txt"))?;
    Ok(())
}

fn main() -> localtrace::Result<()> {
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    match fs_err() {
        Ok(_) => println!("No error occurred."),
        Err(e) => {
            assert_eq!(
                e.to_string(),
                format!(
                    r#"message: No such file or directory (os error 2)
- {cargo_manifest_dir}/src/main_err.rs:3
- {cargo_manifest_dir}/src/main_err.rs:9
"#,
                )
            );
        }
    }

    println!("example completed successfully!");

    Ok(())
}
