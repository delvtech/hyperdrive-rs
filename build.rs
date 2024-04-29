use std::{
    fs,
    io::{self, Write},
    path::Path,
};

fn main() -> io::Result<()> {
    // Read the Cargo.toml file
    let cargo_toml = fs::read_to_string("Cargo.toml")?;

    // Parse the version from the workspace
    let version = cargo_toml
        .lines()
        .find(|line| line.starts_with("version"))
        .map(|line| line.trim().split('"').nth(1).unwrap_or(""))
        .unwrap();

    // Parse the members from the workspace
    let members = cargo_toml
        .lines()
        .skip_while(|line| !line.starts_with("members = ["))
        .skip(1)
        .take_while(|line| !line.starts_with(']'))
        .filter_map(|line| {
            if let Some(member) = line.trim().split("/").nth(1) {
                Some(member.trim_matches(|c| c == '"' || c == ',').to_string())
            } else {
                None
            }
        });

    // Update the Cargo.toml files for each member
    for member in members {
        let member_path = format!("{}{}", "crates/", member);
        let member_toml_path = Path::new(&member_path).join("Cargo.toml");
        let mut member_toml = fs::read_to_string(&member_toml_path)?;
        let member_version = member_toml
            .lines()
            .find(|line| line.starts_with("version"))
            .map(|line| line.trim().split('"').nth(1).unwrap_or(""))
            .unwrap();

        // Replace the version field
        member_toml = member_toml.replace(
            &format!("version = \"{}\"", member_version),
            &format!("version = \"{}\"", version),
        );

        fs::write(&member_toml_path, member_toml)?;
    }

    Ok(())
}
