# Error Handling

```lp
# Result<T, E> - no exceptions
fn read_file(path: String) -> Result<String, Error> {
    match File::open(path) {
        Ok(mut f) => Ok(f.read_to_string()?),
        Err(e) => Err(e),
    }
}

# ? operator for propagation
fn main() -> Result<(), Error> {
    let content = read_file("config.toml")?;
    print(content);
    Ok(())
}
```