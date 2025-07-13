use localtrace::{catch_with_trace, trace, Result};

fn might_fail() -> Result<String> {
    let content = std::fs::read_to_string("config.txt")?;

    if content.is_empty() {
        return trace!("Configuration file is empty");
    }

    Ok(content)
}

fn main() {
    catch_with_trace(|| {
        let config = might_fail()?;
        println!("Config: {config}");
        Ok(())
    });
}
