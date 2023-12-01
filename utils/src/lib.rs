pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    EnvError(std::env::VarError),
    ReqwestError(reqwest::Error),
}

impl From<std::io::Error> for Error {
    fn from(res: std::io::Error) -> Self {
        Self::IoError(res)
    }
}

impl From<std::env::VarError> for Error {
    fn from(res: std::env::VarError) -> Self {
        Self::EnvError(res)
    }
}

impl From<reqwest::Error> for Error {
    fn from(res: reqwest::Error) -> Self {
        Self::ReqwestError(res)
    }
}

pub fn get_input(year: usize, day: usize) -> Result<String> {
    let use_test_input = std::env::args()
        .any(|str| matches!(str.to_lowercase().as_str(), "-t" | "--test"));
    let mode_char = if use_test_input { 't' } else { 'f' };
    let code_block_index = std::env::args().find(|str| str.starts_with("-n"))
        .and_then(|s| s[2..].parse::<usize>().ok())
        .unwrap_or_default();

    let file = std::path::PathBuf::from(format!("./day{day}/input.txt"));
    if file.try_exists().unwrap_or(false) {
        if let Ok(input) = std::fs::read_to_string(file.clone()) {
            let mut chars = input.chars();
            if chars.next().unwrap() == mode_char && chars.next().unwrap() == char::from_digit(code_block_index as u32, 10).unwrap() {
                return Ok(input.chars()
                    .skip(4)
                    .collect::<String>());
            }
        }
    }

    let input_data = if !use_test_input {
        download_input_data(year, day)?
    } else {
        download_test_input(year, day, code_block_index)?
    };

    let contents = format!("{mode_char}{code_block_index}\n\n{input_data}");
    if std::fs::write(file.clone(), contents).is_err() {
        println!("WARNING: Could not write file {file:?}");
    }

    Ok(input_data)
}

fn download_input_data(year: usize, day: usize) -> Result<String> {
    let session_token = std::env::var("AOC_SESSION").map_err(Error::from)?;
    println!("Downloading input data for day {day} ({year})...\n");

    let input_data = reqwest::blocking::Client::new()
        .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .header("cookie", session_token)
        .send().map_err(Error::from)?
        .text().map_err(Error::from)?;

    Ok(input_data)
}

fn download_test_input(year: usize, day: usize, code_block_index: usize) -> Result<String> {
    let session_token = std::env::var("AOC_SESSION").map_err(Error::from)?;
    println!("Downloading test input data for day {day} ({year}) (code block index: {code_block_index})...\n");

    let page = reqwest::blocking::Client::new()
        .get(format!("https://adventofcode.com/{year}/day/{day}"))
        .header("cookie", session_token)
        .send().map_err(Error::from)?
        .text().map_err(Error::from)?;

    let document = scraper::Html::parse_document(&page);
    let selector = scraper::Selector::parse("code").unwrap();
    let element = document.select(&selector).nth(code_block_index).unwrap();
    Ok(element.inner_html())
}
