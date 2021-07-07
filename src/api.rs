use exitfailure::ExitFailure;
use reqwest::Url;
use std::fmt;
use std::fmt::*;
use std::result::Result;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DuckDuckGoQuery {
    pub(crate) Abstract: String,
    pub(crate) AbstractText: String,
    pub(crate) AbstractSource: String,
    pub(crate) AbstractURL: String,
    pub(crate) Image: String,
    pub(crate) Heading: String,
    pub(crate) Answer: String,
    pub(crate) AnswerType: String,
    pub(crate) Definition: String,
    pub(crate) DefinitionSource: String,
    pub(crate) DefinitionURL: String,
    pub(crate) Type: String,
}

impl DuckDuckGoQuery {
    pub async fn new(question: &str) -> std::result::Result<DuckDuckGoQuery, ExitFailure> {
        let question = question.trim().replace(" ", "+");
        let url = format!(
            "https://api.duckduckgo.com/?q={}?&format=json&pretty=1",
            question
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<DuckDuckGoQuery>().await?;
        Ok(res)
    }

    pub fn blank_query() -> DuckDuckGoQuery {
        DuckDuckGoQuery {
            Abstract: String::default(),
            AbstractText: String::default(),
            AbstractSource: String::default(),
            AbstractURL: String::default(),
            Image: String::default(),
            Heading: String::default(),
            Answer: String::default(),
            AnswerType: String::default(),
            Definition: String::default(),
            DefinitionSource: String::default(),
            DefinitionURL: String::default(),
            Type: String::default(),
        }
    }
}

impl Display for DuckDuckGoQuery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();
        if !&self.Answer.is_empty() {
            res += &*(format!("\nAnswer: {}\n", self.Answer.clone()));
        }

        if !&self.Abstract.is_empty() {
            res += &*(format!("\nAbstract: {}\n", self.Abstract.clone()));
        }

        if !&self.Definition.is_empty() {
            res += &*(format!("\nDefinition: {}\n", self.Definition.clone()));
        }

        write!(f, "{}", res)
    }
}
