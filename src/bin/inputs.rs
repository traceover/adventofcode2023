use anyhow::{anyhow, Context, Result};
use minreq;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn main() -> Result<()> {
    let input_dir = DEFAULT_INPUT_PATH;

    if !Path::new(&input_dir).exists() {
        fs::create_dir_all(&input_dir)
            .with_context(|| format!("Could not create inputs directory: {}", input_dir))?;
    }

    let mut cookie = String::new();
    println!("Please enter your session cookie:");
    io::stdin()
        .read_line(&mut cookie)
        .context("Could not read cookie")?;

    for day in 1..=CURRENT_DAY {
        let file_path = format!("{}/{}.txt", input_dir, day);
        let mut file = File::create(&file_path)
            .with_context(|| format!("Could not create file: {}", file_path))?;
        let input = download_input(day, "2023", &cookie)?;
        file.write_all(input.as_bytes())
            .context("Could not write file")?;
    }

    println!("Successfully downloaded inputs for days 1..{}", CURRENT_DAY);
    Ok(())
}

pub const CURRENT_DAY: usize = 8;
pub const DEFAULT_INPUT_PATH: &str = "inputs";

pub fn download_input(day: usize, year: &str, cookie: &str) -> Result<String> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");

    let response = minreq::get(&url)
        .with_header("Cookie", format!("session={cookie}"))
        .send()
        .with_context(|| format!("Could not send request: {}", url))?;

    let body = response
        .as_str()
        .context(format!("Could not read response"))?;

    if response.status_code != 200 {
        return Err(anyhow!(format!(
            "Response status code {}",
            response.status_code
        )));
    }

    assert_eq!("OK", response.reason_phrase);

    Ok(body.to_string())
}
