
use http::httprequest::HttpRequest;
use http::httpresponse::HttpResponse;
use serde::{Serialize, Deserialize};
use std::{default, env, fs};
use std:fs;


pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;
    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);
        let contents = fs::read_to_string(full_path);
        contents.ok()
    }
}


#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}
 
pub struct StaticPageHandler;
 
pub struct PageNotFoundHandler;
impl Handler for PageNotFoundHandler {
    fn handle(_req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}
 
pub struct WebServiceHandler;