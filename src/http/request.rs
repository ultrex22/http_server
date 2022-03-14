pub mod request {

    use crate::http;

    pub struct Request {
        path: String,
        query_string: Option<String>,
        method: http::Method,
    }
}