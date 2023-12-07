use minreq;
use anyhow::{anyhow, Result, Context};

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
        return Err(anyhow!(format!("Response status code {}", response.status_code)));
    }

    assert_eq!("OK", response.reason_phrase);

    Ok(body.to_string())
}
