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
            ContentType::HTML => "text/html",
            ContentType::CSS => "text/css",
            ContentType::JS => "application/javascript",
            ContentType::IMAGE => "image/jpeg",
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
        "{status_line}\r\nContent-Type: {html_type}\r\nContent-Length: {len}\r\n\r\n",
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
    fn test_format_response() {
        let status_line = "status";
        let html_type = "content";
        let body = vec![1, 2, 3];
        let mut expected = format!(
            "{status_line}\r\nContent-Type: {html_type}\r\nContent-Length: {len}\r\n\r\n",
            status_line = status_line,
            html_type = html_type,
            len = body.len(),
        )
        .into_bytes();
        expected.extend(body.clone());

        assert_eq!(expected, format_response(status_line, html_type, body));
    }

    #[test]
    fn test_bytes_from_html_response() {
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
    fn test_bytes_from_css_response() {
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
    fn test_bytes_from_js_response() {
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
    fn test_bytes_from_image_response() {
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

    #[test]
    fn test_str_from_html() {
        let content_type: &str = ContentType::HTML.into();
        assert_eq!("text/html", content_type);
    }

    #[test]
    fn test_str_from_js() {
        let content_type: &str = ContentType::JS.into();
        assert_eq!("application/javascript", content_type);
    }

    #[test]
    fn test_str_from_css() {
        let content_type: &str = ContentType::CSS.into();
        assert_eq!("text/css", content_type);
    }

    #[test]
    fn test_str_from_image() {
        let content_type: &str = ContentType::IMAGE.into();
        assert_eq!("image/jpeg", content_type);
    }
}
