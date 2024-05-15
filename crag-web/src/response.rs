pub enum Response {
    Ok(String),
    NotFound(String),
}
const HTML_TYPE: &str = "Content-Type: text/html";
impl From<Response> for Vec<u8> {
    fn from(res: Response) -> Vec<u8> {
        match res {
            Response::Ok(body) => {
                const STATUS_LINE: &str = "HTTP/1.0 200 OK";
                format_response(STATUS_LINE, HTML_TYPE, body)
            }
            Response::NotFound(_) => {
                const STATUS_LINE: &str = "HTTP/1.0 404 Not Found";
                const BODY: &str = include_str!("../static/html/404.html");
                format_response(STATUS_LINE, HTML_TYPE, BODY.to_owned())
            }
        }
        .into_bytes()
    }
}

fn format_response(status_line: &str, html_type: &str, body: String) -> String {
    format!(
        "{status_line}\r\n{html_type}Content-Length: {len}\r\n\r\n{body}",
        status_line = status_line,
        html_type = html_type,
        len = body.len(),
        body = body
    )
}
