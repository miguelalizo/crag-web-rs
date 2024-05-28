use lazy_static::lazy_static;
use std::collections;

pub enum Response<T: IntoBytes> {
    Ok(T, ContentType),
    NotFound(T),
}

#[derive(Hash, Eq, PartialEq)]
pub enum ContentType {
    HTML,
    CSS,
    JS,
    IMAGE,
}

pub trait IntoBytes {
    fn into_bytes(self) -> Vec<u8>;
}

impl IntoBytes for String {
    fn into_bytes(self) -> Vec<u8> {
        self.into_bytes()
    }
}

impl IntoBytes for &str {
    fn into_bytes(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl IntoBytes for Vec<u8> {
    fn into_bytes(self) -> Vec<u8> {
        self
    }
}

lazy_static! {
    static ref CONTENT_TYPE: collections::HashMap<ContentType, &'static str> = {
        let mut map = collections::HashMap::new();
        map.insert(ContentType::HTML, "Content-Type: text/html");
        map.insert(ContentType::CSS, "Content-Type: text/css");
        map.insert(ContentType::JS, "Content-Type: application/javascript");
        map.insert(ContentType::IMAGE, "Content-Type: image/jpeg");
        map
    };
}
// const HTML_TYPE: &str = "Content-Type: text/html";

impl<T: IntoBytes> From<Response<T>> for Vec<u8> {
    fn from(res: Response<T>) -> Vec<u8> {
        match res {
            Response::Ok(body, content_type) => {
                const STATUS_LINE: &str = "HTTP/1.0 200 OK";
                format_response(
                    STATUS_LINE,
                    CONTENT_TYPE.get(&content_type).unwrap(),
                    body.into_bytes(),
                )
            }
            Response::NotFound(body) => {
                const STATUS_LINE: &str = "HTTP/1.0 404 Not Found";
                // const BODY: &str = include_str!("../static/html/404.html");
                format_response(
                    STATUS_LINE,
                    CONTENT_TYPE.get(&ContentType::HTML).unwrap(),
                    body.into_bytes(),
                )
            }
        }
    }
}

fn format_response(status_line: &str, html_type: &str, body: Vec<u8>) -> Vec<u8> {
    let mut response = format!(
        "{status_line}\r\n{html_type}\r\nContent-Length: {len}\r\n\r\n",
        status_line = status_line,
        html_type = html_type,
        len = body.len(),
    )
    .into_bytes();

    response.extend(body);
    response
}
