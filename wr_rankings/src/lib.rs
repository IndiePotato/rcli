#![allow(dead_code)]

use std::fmt::Formatter;

use colored::Colorize;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RankingsResponse {
    label: String,
    entries: Vec<InternationalRanking>,
    effective: Effective,
}

#[derive(Debug, Deserialize)]
pub struct InternationalRanking {
    team: InternationalTeam,
    pts: f32,
    pos: i8,
    #[allow(non_snake_case)]
    previousPts: f32,
    #[allow(non_snake_case)]
    previousPos: i8,
}

impl std::fmt::Display for InternationalRanking {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let delta = self.previousPos - self.pos;
        print!("{}. {} - {}", self.pos, self.team.name, self.pts);
        if delta < 0 {
            print!(" {}", delta.to_string().red());
        } else if delta > 0 {
            print!(" +{}", delta.to_string().bright_green());
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct InternationalTeam {
    id: String,
    #[allow(non_snake_case)]
    altId: String,
    name: String,
    abbreviation: String,
    #[allow(non_snake_case)]
    countryCode: String,
    annotations: Option<String>,
    metadata: Metadata,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {}

#[derive(Debug, Deserialize)]
pub struct Effective {
    millis: usize,
    #[allow(non_snake_case)]
    gmtOffset: f64,
    label: String,
}

#[derive(Debug)]
pub enum GenderParam {
    MRU,
    WRU,
}

impl GenderParam {
    fn as_str(&self) -> &str {
        match self {
            GenderParam::MRU => "mru",
            GenderParam::WRU => "wru",
        }
    }
}

impl std::fmt::Display for GenderParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn get_int_rankings(gender: GenderParam, max: u8) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}{}{}",
        "https://api.wr-rims-prod.pulselive.com/rugby/v3/rankings/",
        gender.as_str(),
        "?language=en"
    );
    let resp = ureq::get(url)
        .call()?
        .body_mut()
        .read_json::<RankingsResponse>()?;
    for team_ranking in resp.entries.iter().take(max.into()) {
        println!("{}", team_ranking);
    }
    Ok(())
}
