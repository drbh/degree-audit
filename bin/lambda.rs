// https://crates.io/crates/lambda_runtime
// This example requires the following input to succeed:
// { "command": "do something" }

use lambda_runtime::{handler_fn, Context, Error};
// use log::LevelFilter;
use serde::{Deserialize, Serialize};
// use simple_logger::SimpleLogger;

use degree_audit::{build_degree, Config, ExactMatch, GroupMatch, Student};
// use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BrickInput {
    pub match_type: MatchType,
    pub group: Option<String>,
    pub subject: Option<String>,
    pub level: Option<i64>,
}

pub type InputCard = Vec<Vec<BrickInput>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Requirement {
    pub original: String,
    pub card: InputCard,
}

pub type DegreeMap = Vec<Vec<Requirement>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Requirements {
    pub original: String,
    pub match_type: MatchType,
    pub card: Vec<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MatchType {
    Exact,
    Group,
    Unknown,
}

pub type MajorMap = Vec<Vec<Requirements>>;
pub type CardConfig = Vec<Vec<Box<dyn Config>>>;

pub fn execute_raw(student: Student, major_map: DegreeMap) -> Vec<logicmap::CardResult> {
    let mut my_cards: Vec<CardConfig> = Vec::new();

    for term in &major_map {
        for req in term {
            let mut statment_configs = Vec::new();
            for card in &req.card {
                // let card_configs: Vec<Box<dyn Config>> = Vec::new();

                let mut translated_bricks: Vec<Box<dyn Config>> = Vec::new();
                for stmt in card {
                    match stmt.match_type {
                        MatchType::Exact => {
                            let sub = stmt.subject.as_ref().unwrap();
                            translated_bricks.push(Box::new(ExactMatch {
                                subject: sub.to_string(),
                                level: stmt.level.unwrap_or(0) as usize,
                            }));
                        }
                        MatchType::Group => {
                            let grp = stmt.group.as_ref().unwrap();
                            translated_bricks.push(Box::new(GroupMatch {
                                group: grp.to_string(),
                            }));
                        }
                        MatchType::Unknown => {}
                    }
                }
                statment_configs.push(translated_bricks);
            }
            my_cards.push(statment_configs);
        }
    }

    let degree = build_degree(student, my_cards);

    let mut degree_result = Vec::new();
    for requirement in degree {
        let _finished_card = requirement.report();
        degree_result.push(_finished_card);
    }
    let _json_degree = json!(degree_result);
    return degree_result;
}

#[derive(Debug, Serialize, Deserialize)]
struct AuditInput {
    map: DegreeMap,
    student: Student,
}

// /// This is also a made-up example. Requests come into the runtime as unicode
// /// strings in json format, which can map to any structure that implements `serde::Deserialize`
// /// The runtime pays no attention to the contents of the request payload.
// #[derive(Deserialize)]
// struct Request {
//     command: String,
// }

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // // required to enable CloudWatch error logging by the runtime
    // // can be replaced with any other method of initializing `log`
    // SimpleLogger::new()
    //     .with_level(LevelFilter::Info)
    //     .init()
    //     .unwrap();

    let func = handler_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn my_handler(event: AuditInput, ctx: Context) -> Result<Response, Error> {
    // extract some useful info from the request
    // let command = event.command;
    let result = execute_raw(event.student.clone(), event.map);

    // prepare the response
    let resp = Response {
        req_id: ctx.request_id,
        // msg: format!("Command {} executed.", event.student.name),
        msg: json!(result).to_string(),
    };

    // return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}
