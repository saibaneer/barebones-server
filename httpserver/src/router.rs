// Import handlers and HTTP-related types
use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*; // For read/write traits


// The Router struct will handle routing logic
pub struct Router;

impl Router {
    // Main routing function that handles HTTP requests and routes them to appropriate handlers
    pub fn router(req: HttpRequest, stream: &mut impl Write) -> () {
        // Match on the HTTP method of the request
        match req.method {
            // If it's a GET request, handle it
            httprequest::Method::GET => Self::handle_get_request(req, stream),
            // For all other HTTP methods (e.g., POST, PUT, DELETE), return a 404 response
            _ => Self::handle_not_found(req, stream),
        }
    }

    // Function to handle GET requests
    fn handle_get_request(req: HttpRequest, stream: &mut impl Write) {
        // Ensure the resource is a path and extract the path string
        let httprequest::Resource::Path(path_string) = &req.resource;
        // Split the path string by '/' and collect it into a vector of &str
        let route = path_string.split("/").collect::<Vec<&str>>();
        println!("path vector is: {:#?}", &route); // Debug output to show the route vector

        // Check if the route has more than one element and if the second element is "api"
        if route.len() > 1 && route[1] == "api" {
            // If the route starts with /api, handle it as an API request
            Self::handle_api(req, stream);
        } else {
            // Otherwise, handle it as a request for a static page
            Self::handle_static(req, stream);
        }
    }

    // Function to handle API requests (e.g., /api/endpoint)
    fn handle_api(req: HttpRequest, stream: &mut impl Write) {
        // Use WebServiceHandler to process the API request and generate a response
        let resp = WebServiceHandler::handle(&req);
        // Send the response to the client
        let _ = resp.send_response(stream);
    }

    // Function to handle static page requests (e.g., /index.html)
    fn handle_static(req: HttpRequest, stream: &mut impl Write) {
        // Use StaticPageHandler to process the request and generate a response for static files
        let resp = StaticPageHandler::handle(&req);
        // Send the static page response to the client
        let _ = resp.send_response(stream);
    }

    // Function to handle requests that don't match any valid route (404 Not Found)
    fn handle_not_found(req: HttpRequest, stream: &mut impl Write) {
        // Use PageNotFoundHandler to generate a 404 Not Found response
        let resp = PageNotFoundHandler::handle(&req);
        // Send the 404 response to the client
        let _ = resp.send_response(stream);
    }
}
