fn ok() -> localtrace::Result<()> {
    std::fs::read_to_string(String::from("Cargo.toml"))?;
    Ok(())
}

fn main() -> localtrace::Result<()> {
    match ok() {
        Ok(_) => println!("No error occurred."),
        Err(e) => {
            panic!("An error occurred: {e}");
        }
    }

    println!("example completed successfully!");

    Ok(())
}
