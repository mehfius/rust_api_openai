use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::env;
use dotenvy::dotenv;

#[derive(Deserialize)]
struct Question {
    question: String,
}

#[derive(Serialize)]
struct ApiResponse {
    answer: String,
    error: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[post("/ask")]
async fn ask_openai(question: web::Json<Question>) -> impl Responder {
    let client = Client::new();
    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    let openai_request = OpenAIRequest {
        model: "gpt-4o-mini".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: question.question.clone(),
        }],
        temperature: 0.7,
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", openai_api_key))
        .json(&openai_request)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<OpenAIResponse>().await {
                    Ok(data) => {
                        let answer = data.choices.get(0)
                            .map(|choice| choice.message.content.clone())
                            .unwrap_or_else(|| "No response from OpenAI".to_string());
                        HttpResponse::Ok().json(ApiResponse {
                            answer,
                            error: None,
                        })
                    }
                    Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                        answer: "".to_string(),
                        error: Some(format!("Failed to parse OpenAI response: {}", e)),
                    }),
                }
            } else {
                HttpResponse::BadGateway().json(ApiResponse {
                    answer: "".to_string(),
                    error: Some(format!("OpenAI API error: {}", resp.status())),
                })
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
            answer: "".to_string(),
            error: Some(format!("Request failed: {}", e)),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port: u16 = env::var("PORT")
        .expect("Variável de ambiente PORT não está preenchida")
        .parse()
        .expect("PORT deve ser um número inteiro");

    HttpServer::new(|| {
        App::new()
            .service(ask_openai)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}