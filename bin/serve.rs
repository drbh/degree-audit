use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use degree_audit::{build_degree, ExactMatch, GroupMatch, Student};
use logicmap::Config;
use serde::{Deserialize, Serialize};
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
pub type CardConfig = Vec<Vec<Box<dyn Config<Student>>>>;

pub fn execute_raw(student: Student, major_map: DegreeMap) -> Vec<logicmap::CardResult> {
    let mut my_cards: Vec<CardConfig> = Vec::new();

    for term in &major_map {
        for req in term {
            let mut statment_configs = Vec::new();
            for card in &req.card {
                // let card_configs: Vec<Box<dyn Config>> = Vec::new();

                let mut translated_bricks: Vec<Box<dyn Config<Student>>> = Vec::new();
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

/// This handler uses json extractor
async fn execute_audit(item: web::Json<AuditInput>) -> HttpResponse {
    // println!("model: {:?}", &item);
    let output = execute_raw(item.student.clone(), item.map.clone());
    HttpResponse::Ok().json(output) // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("Degree Audit Server is running at 127.0.0.1:9966");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/audit").route(web::post().to(execute_audit)))
    })
    .bind("127.0.0.1:9966")?
    .run()
    .await
}
