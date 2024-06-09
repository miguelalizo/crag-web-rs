use crag_web::methods;
use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;

use crag_web::request;
use crag_web::response;
use crag_web::server;

const STATIC_FILES: &str = "./static/";

// GET /not_found
fn not_found(_req: request::Request) -> anyhow::Result<response::Response> {
    let filename = format!("{STATIC_FILES}html/404.html");
    let html = std::fs::read(filename).unwrap();
    Ok(response::Response::Ok(html, response::ContentType::HTML))
}

// GET /index
fn index(_req: request::Request) -> anyhow::Result<response::Response> {
    let filename = format!("{STATIC_FILES}html/index.html");
    let html = std::fs::read(filename).unwrap();
    Ok(response::Response::Ok(html, response::ContentType::HTML))
}

fn send_email() {
    // Set up SMTP credentials (your Gmail address and password)
    let email = "some_email";
    let password = "some_password";
    let smtp_server = "smtp.gmail.com";

    // Create SMTP client with SSL
    let _smtp_client = SmtpTransport::starttls_relay(smtp_server)
        .unwrap()
        .credentials(Credentials::new(email.to_string(), password.to_string()))
        .build();

    // // Define the email
    // let email = Message::builder()
    //     .from("som_email".parse().unwrap())
    //     .to("some_email".parse().unwrap())
    //     .subject("Rust Email")
    //     .body(String::from("Hello, this is a test email from Rust!"))
    //     .unwrap();

    // // Send the email
    // match smtp_client.send(&email) {
    //     Ok(_) => println!("Email sent successfully!"),
    //     Err(e) => eprintln!("Failed to send email: {:?}", e),
    // }
}

// GET /contact
// POST /contact
fn contact(req: request::Request) -> anyhow::Result<response::Response> {
    let filename = match req.method {
        methods::Method::POST => {
            // println!("{}", body);
            send_email();
            format!("{STATIC_FILES}html/thanks.html")
        }
        _ => format!("{STATIC_FILES}html/contact.html"),
    };
    let html = std::fs::read(filename).unwrap();
    Ok(response::Response::Ok(html, response::ContentType::HTML))
}

// GET /css/default.css
fn css_default(_req: request::Request) -> anyhow::Result<response::Response> {
    let filename = format!("{STATIC_FILES}css/default.css");
    let css = std::fs::read(filename).unwrap(); //?;;
    Ok(response::Response::Ok(css, response::ContentType::CSS))
}

// GET /css/blue.css
fn css_blue(_req: request::Request) -> anyhow::Result<response::Response> {
    let filename = format!("{STATIC_FILES}css/blue.css");
    let css = std::fs::read(filename).unwrap(); //?;;
    Ok(response::Response::Ok(css, response::ContentType::CSS))
}

// GET /css/green.css
fn css_green(_req: request::Request) -> anyhow::Result<response::Response> {
    let filename = format!("{STATIC_FILES}css/green.css");
    let css = std::fs::read(filename).unwrap(); //?;;
    Ok(response::Response::Ok(css, response::ContentType::CSS))
}

// GET /css/purple.css
fn css_purple(_req: request::Request) -> anyhow::Result<response::Response> {
    let filename = format!("{STATIC_FILES}css/purple.css");
    let css = std::fs::read(filename).unwrap(); //?;;
    Ok(response::Response::Ok(css, response::ContentType::CSS))
}

// GET /scripts/script.js
fn js_script(_req: request::Request) -> anyhow::Result<response::Response> {
    let filename = format!("{STATIC_FILES}scripts/script.js");
    let script = std::fs::read(filename).unwrap(); //?;;
    Ok(response::Response::Ok(script, response::ContentType::JS))
}

// GET /image/me.jpeg
fn image_me(_req: request::Request) -> anyhow::Result<response::Response> {
    let filename = format!("{STATIC_FILES}images/me.jpeg");
    let image = std::fs::read(filename).unwrap(); //?;;
    Ok(response::Response::Ok(image, response::ContentType::IMAGE))
}

// GET /image/404.jpeg
fn image_404(_req: request::Request) -> anyhow::Result<response::Response> {
    let filename = format!("{STATIC_FILES}images/404.jpeg");
    let image = std::fs::read(filename).unwrap(); //?;;
    Ok(response::Response::Ok(image, response::ContentType::IMAGE))
}

// GET /image/linkedin.jpeg
fn image_linkedin(_req: request::Request) -> anyhow::Result<response::Response> {
    let filename = format!("{STATIC_FILES}images/linkedin.jpeg");
    let image = std::fs::read(filename).unwrap(); //?;;
    Ok(response::Response::Ok(image, response::ContentType::IMAGE))
}

// GET /image/linkedin.jpeg
fn image_mail_sent(_req: request::Request) -> anyhow::Result<response::Response> {
    let filename = format!("{STATIC_FILES}images/mail_sent.jpeg");
    let image = std::fs::read(filename).unwrap(); //?;;
    Ok(response::Response::Ok(image, response::ContentType::IMAGE))
}

// GET <bad request>
fn error_404(_req: request::Request) -> anyhow::Result<response::Response> {
    let filename = format!("{STATIC_FILES}html/404.html");
    let html = std::fs::read(filename).unwrap(); //?;;
    Ok(response::Response::NotFound(html))
}

fn main() -> anyhow::Result<()> {
    let srvr = server::Server::build()
        .register_error_handler(error_404)?
        .register_handler("/images/404.jpeg".into(), image_404)?
        .register_handler("/".into(), index)?
        .register_handler("/contact".into(), contact)?
        .register_handler("/not_found".into(), not_found)?
        .register_handler("/scripts/script.js".into(), js_script)?
        .register_handler("/css/default.css".into(), css_default)?
        .register_handler("/css/blue.css".into(), css_blue)?
        .register_handler("/css/green.css".into(), css_green)?
        .register_handler("/css/purple.css".into(), css_purple)?
        .register_handler("/images/me.jpeg".into(), image_me)?
        .register_handler("/images/linkedin.jpeg".into(), image_linkedin)?
        .register_handler("/images/mail_sent.jpeg".into(), image_mail_sent)?
        .finalize(("127.0.0.1", 8010), 4)
        .unwrap();

    // run Server
    srvr.run()?;

    Ok(())
}
