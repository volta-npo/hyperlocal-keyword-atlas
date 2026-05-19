use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use hyperlocal_keyword_atlas_backend::{
    blocks_release, release_fingerprint, release_score, sample_gates, sample_keywords,
    score_keyword, PRODUCT_SLUG, PRODUCT_TITLE,
};

fn response(status: &str, body: &str) -> String {
    format!("HTTP/1.1 {status}\r\nContent-Type: application/json; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len())
}

fn score_body() -> String {
    let gates = sample_gates();
    let keyword_scores = sample_keywords().iter().map(score_keyword).map(|score| {
        format!(
            r#"{{"term":"{}","service":"{}","geography":"{}","intent":"{}","confidence":{},"priority":"{}","warnings":{}}}"#,
            score.term,
            score.service,
            score.geography,
            score.intent,
            score.confidence,
            score.priority,
            json_string_array(&score.warnings)
        )
    }).collect::<Vec<_>>().join(",");
    format!(
        r#"{{"product":"{}","score":{},"blocked":{},"fingerprint":"{}","keywords":[{}]}}"#,
        PRODUCT_SLUG,
        release_score(&gates),
        blocks_release(&gates),
        release_fingerprint(&gates),
        keyword_scores
    )
}

fn json_string_array(values: &[String]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| format!(r#""{}""#, value.replace('"', "\\\"")))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn handle(mut stream: TcpStream) {
    let mut buffer = [0; 8192];
    let _ = stream.read(&mut buffer);
    let request = String::from_utf8_lossy(&buffer);
    let body = if request.starts_with("GET /health") || request.starts_with("GET / ") {
        format!(
            r#"{{"ok":true,"product":"{}","title":"{}"}}"#,
            PRODUCT_SLUG, PRODUCT_TITLE
        )
    } else if request.starts_with("GET /score") || request.starts_with("POST /score") {
        score_body()
    } else {
        let not_found = r#"{"ok":false,"error":"not_found"}"#;
        let _ = stream.write_all(response("404 Not Found", not_found).as_bytes());
        return;
    };
    let _ = stream.write_all(response("200 OK", &body).as_bytes());
}

fn serve() -> std::io::Result<()> {
    let addr = env::var("VOLTA_BACKEND_ADDR").unwrap_or_else(|_| "127.0.0.1:8788".to_string());
    let listener = TcpListener::bind(&addr)?;
    println!("{} backend listening on http://{}", PRODUCT_TITLE, addr);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle(stream),
            Err(err) => eprintln!("connection error: {err}"),
        }
    }
    Ok(())
}

fn main() {
    if env::args().any(|arg| arg == "serve") {
        serve().expect("backend server failed");
        return;
    }
    let gates = sample_gates();
    println!("{} ({})", PRODUCT_TITLE, PRODUCT_SLUG);
    println!("score={}", release_score(&gates));
    println!("blocked={}", blocks_release(&gates));
    println!("fingerprint={}", release_fingerprint(&gates));
    for score in sample_keywords().iter().map(score_keyword) {
        println!(
            "keyword={} confidence={} priority={}",
            score.term, score.confidence, score.priority
        );
    }
}
