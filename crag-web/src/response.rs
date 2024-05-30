pub enum Response {
    Ok(Vec<u8>, ContentType),
    NotFound(Vec<u8>),
}

pub enum ContentType {
    HTML,
    CSS,
    JS,
    IMAGE,
}

impl From<ContentType> for &'static str {
    fn from(content_type: ContentType) -> &'static str {
        match content_type {
            ContentType::HTML => "Content-Type: text/html",
            ContentType::CSS => "Content-Type: text/css",
            ContentType::JS => "Content-Type: application/javascript",
            ContentType::IMAGE => "Content-Type: image/jpeg",
        }
    }
}

impl From<Response> for Vec<u8> {
    fn from(res: Response) -> Vec<u8> {
        match res {
            Response::Ok(body, content_type) => {
                const STATUS_LINE: &str = "HTTP/1.0 200 OK";
                format_response(STATUS_LINE, content_type.into(), body)
            }
            Response::NotFound(body) => {
                const STATUS_LINE: &str = "HTTP/1.0 404 Not Found";
                format_response(STATUS_LINE, ContentType::HTML.into(), body)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_html_response() {
        let body = vec![1, 2, 3];
        let response = Response::Ok(body.clone(), ContentType::HTML);
        let expected = format!(
            "HTTP/1.0 200 OK\r\nContent-Type: text/html\r\nContent-Length: {len}\r\n\r\n",
            len = body.len()
        )
        .into_bytes();
        let mut expected = expected;
        expected.extend(&body);
        assert_eq!(Vec::<u8>::from(response), expected);

        let response = Response::NotFound(body.clone());
        let expected = format!(
            "HTTP/1.0 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: {len}\r\n\r\n",
            len = body.len()
        )
        .into_bytes();
        let mut expected = expected;
        expected.extend(body);
        assert_eq!(Vec::<u8>::from(response), expected);
    }

    #[test]
    fn test_from_css_response() {
        let body = vec![1, 2, 3];
        let response = Response::Ok(body.clone(), ContentType::CSS);

        let expected = format!(
            "HTTP/1.0 200 OK\r\nContent-Type: text/css\r\nContent-Length: {len}\r\n\r\n",
            len = body.len()
        )
        .into_bytes();
        let mut expected = expected;
        expected.extend(&body);

        assert_eq!(Vec::<u8>::from(response), expected);
    }

    #[test]
    fn test_from_js_response() {
        let body = vec![1, 2, 3];
        let response = Response::Ok(body.clone(), ContentType::JS);

        let expected = format!(
            "HTTP/1.0 200 OK\r\nContent-Type: application/javascript\r\nContent-Length: {len}\r\n\r\n",
            len = body.len()
        )
        .into_bytes();
        let mut expected = expected;
        expected.extend(&body);

        assert_eq!(Vec::<u8>::from(response), expected);
    }

    #[test]
    fn test_from_image_response() {
        let body = vec![1, 2, 3];
        let response = Response::Ok(body.clone(), ContentType::IMAGE);

        let expected = format!(
            "HTTP/1.0 200 OK\r\nContent-Type: image/jpeg\r\nContent-Length: {len}\r\n\r\n",
            len = body.len()
        )
        .into_bytes();
        let mut expected = expected;
        expected.extend(&body);

        assert_eq!(Vec::<u8>::from(response), expected);
    }
}
