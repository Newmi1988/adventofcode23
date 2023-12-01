use std::{error::Error, fs};

use toml_edit::{Document, Item, Value};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    project: String,
}

fn update_toml(entry: &str) -> Result<(), Box<dyn Error>> {
    let cargo_config = include_str!("../../Cargo.toml");
    let mut doc = cargo_config.parse::<Document>()?;
    let arr = toml_edit::Array::new();
    let mut members = match doc["workspace"]["members"].as_array().cloned() {
        Some(a) => a,
        _ => arr,
    };
    if members.is_empty() {
        return Err("Entry members not found".into());
    } else {
        // let formatted_value = entry.parse::<toml_edit::Value>().unwrap();
        let entry_value: Value = entry.into();
        members.push_formatted(entry_value)
    }
    doc["workspace"]["members"] = Item::Value(Value::from(members));
    fs::write("Cargo.toml", doc.to_string())?;

    Ok(())
}

fn main() {
    let args = Args::parse();

    update_toml(&args.project).expect("Error while updating config")
}
