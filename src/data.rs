use lazy_static::lazy_static;
use crate::domain::{HBOIExampleResponse, HBOIResponseBody, VaardighedenResponseBody};

const SKILL_CONTENT: &'static str = include_str!("./../app/data/vaardigheden-nl.json");
lazy_static! {
    pub static ref SKILL_DATA: VaardighedenResponseBody = serde_json::from_str::<VaardighedenResponseBody>(SKILL_CONTENT).unwrap();
}

const HBOI_CONTENT: &'static str = include_str!("./../app/data/hboi-nl.json");
lazy_static! {
    pub static ref HBOI_DATA: HBOIResponseBody = serde_json::from_str::<HBOIResponseBody>(HBOI_CONTENT).unwrap();
}

const EXAMPLES_CONTENT: &'static str = include_str!("./../app/data/examples.json");
lazy_static! {
    pub static ref EXAMPLES_DATA: Vec<HBOIExampleResponse> = serde_json::from_str::<Vec<HBOIExampleResponse>>(EXAMPLES_CONTENT).unwrap();
}