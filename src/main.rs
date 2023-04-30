use reqwest::get;

#[tokio::main]
async fn main() {
    let line = "Chen, C., Lee, S.-Y., & Stevenson, H. W. (1995). Response style and cross-cultural comparisons of rating scales among East Asian and North American students. Psychological Science, 6(3), 170–175. https://doi.org/10.1111/j.1467-.1995.tb00327.x";
    println!("{}",add_existence_badge(line).await);
}

async fn add_existence_badge(line: &str) -> String {
    let url = parse_line(line);
    let mut result = String::from("");
    if exist_doi(url).await.unwrap() {
        result += "[OK]";
        result += line;
        return result;
    } else {
        result += "[NG]";
        result += line;
        return result;
    }
}

fn parse_line(line: &str) -> &str {
    let mut url = "";
    for word in line.split_whitespace() {
        if word.starts_with("https://doi.org/") || word.starts_with("http://dx.doi.org") {
            url = word;
        }
    }
    url
}

async fn exist_doi(url: &str) -> reqwest::Result<bool> {
    let body = get(url).await?;
    return Ok(!(body.status() == 404));
}

#[cfg(test)]
mod tests {
    use crate::{parse_line, exist_doi};
    // use super::*;
    use mockito::mock;
    use tokio::runtime::Runtime;

    #[test]
    fn test_parse_url() {
        let line = "Chen, C., Lee, S.-Y., & Stevenson, H. W. (1995). Response style and cross-cultural comparisons of rating scales among East Asian and North American students. Psychological Science, 6(3), 170–175. https://doi.org/10.1111/j.1467-9280.1995.tb00327.x";
        assert_eq!(parse_line(line), "https://doi.org/10.1111/j.1467-9280.1995.tb00327.x");
    }

    #[test]
    fn test_exist_doi() {
        let _m = mock("GET", "/")
            .with_status(200)
            .with_body("doi exists")
            .create();

        let url = &mockito::server_url();
        let rt = Runtime::new().unwrap();

        let exist = rt.block_on(exist_doi(url)).unwrap();

        assert_eq!(exist, true);
    }
}