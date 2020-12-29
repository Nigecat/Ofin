use super::TokenMatcher;
use crate::util;
use serde::Deserialize;
use toml::Value;

#[derive(Deserialize, Debug)]
struct OfinSyntaxData {
    matcher: String,
    replace_with: Option<String>,
    extractor: Option<String>,
    preprocess: Option<bool>,
}

pub fn syntax() -> Vec<TokenMatcher> {
    // Token matchers should preferably go in the `syntax.toml` file
    // These are only here since they require assistance from rust code
    let mut matchers: Vec<TokenMatcher> = vec![TokenMatcher::new(
        "ImportStatement".to_string(),
        r#"import\s*<.*>"#.to_string(),
        "use $1".to_string(),
        false,
        Some("<.*>".to_string()),
        Some(|s| {
            format!(
                "ofin_{}::*",
                util::remove_last(util::remove_first(&s.replace("/", "::")).unwrap()).unwrap()
            )
        }),
    )];

    let source = include_str!("../syntax.toml");
    let syntax = source.parse::<Value>().unwrap();
    let syntax = syntax.as_table().unwrap();

    let tokens = &syntax["Token"].as_table().unwrap();

    for token in tokens.iter() {
        let (name, data) = token;
        let data = toml::to_string(data).unwrap();
        let mut data: OfinSyntaxData = toml::from_str(&data).unwrap();

        debug!("Parsed syntax for token {:?} with data {:?}", name, data);

        let mut mutator: Option<fn(&str) -> String> = None;

        if data.extractor == Some("copy".to_string()) {
            data.extractor = Some(data.matcher.clone());
        }

        if data.replace_with == Some("self".to_string()) {
            data.replace_with = Some("$1".to_string());
            data.extractor = Some(data.matcher.clone());
            mutator = Some(|s: &str| s.to_string());
        }

        matchers.push(TokenMatcher::new(
            name.to_string(),
            data.matcher,
            data.replace_with.unwrap_or_else(|| "".to_string()),
            data.preprocess.unwrap_or(false),
            data.extractor,
            mutator,
        ));
    }

    debug!("Finished parsing syntax...\n");
    matchers
}
