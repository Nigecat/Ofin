use super::TokenMatcher;
use serde::Deserialize;
use toml::Value;

#[derive(Deserialize, Debug)]
struct OfinSyntaxData {
    matcher: String,
    replace_with: Option<String>,
    extractor: Option<String>,
}

pub fn syntax() -> Vec<TokenMatcher> {
    let mut matchers: Vec<TokenMatcher> = Vec::new();

    let source = include_str!("../syntax.toml");
    let syntax = source.parse::<Value>().unwrap();
    let syntax = syntax.as_table().unwrap();

    let tokens = &syntax["Token"].as_table().unwrap();

    for token in tokens.iter() {
        let (name, data) = token;
        let data = toml::to_string(data).unwrap();
        let mut data: OfinSyntaxData = toml::from_str(&data).unwrap();

        trace!("Parsed syntax for token {:?} with data {:?}", name, data);

        let mut mutator: Option<fn(&str) -> &str> = None;

        if data.extractor == Some("copy".to_string()) {
            data.extractor = Some(data.matcher.clone());
        }

        if data.replace_with == Some("self".to_string()) {
            data.replace_with = Some("$1".to_string());
            data.extractor = Some(data.matcher.clone());
            mutator = Some(|s: &str| { s });
        }

        matchers.push(TokenMatcher::new(
            name.to_string(),
            data.matcher,
            data.replace_with.unwrap_or_else(|| "".to_string()),
            data.extractor,
            mutator,
        ));
    }

    trace!("Finished parsing syntax...\n");
    matchers
}
