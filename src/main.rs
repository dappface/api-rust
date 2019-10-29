mod slack;

use futures_util::try_stream::TryStreamExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use regex::Regex;
use slack::{PayloadBuilder, SectionBuilder, Slack};
use std::collections::HashMap;
use std::env;
use url::form_urlencoded;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

static NOTFOUND: &[u8] = b"Not Found";

fn validate_params(
    params: &HashMap<String, String>,
) -> std::result::Result<(String, String, String), String> {
    match (
        params.get("name"),
        params.get("email"),
        params.get("message"),
    ) {
        (Some(name), Some(email), Some(message)) => {
            match (name.is_empty(), email.is_empty(), message.is_empty()) {
                (false, false, false) => {
                    let parts: Vec<&str> = email.rsplitn(2, "@").collect();
                    let email_user_re =
                        Regex::new(r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+\z").unwrap();
                    let email_domain_re = Regex::new(r"(?i)^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$").unwrap();

                    match (
                        email_user_re.is_match(parts[0]),
                        email_domain_re.is_match(parts[1]),
                    ) {
                        (true, true) => {
                            return Ok((name.to_string(), email.to_string(), message.to_string()));
                        }
                        _ => {
                            return Err("Invalid email address".to_string());
                        }
                    }
                }
                _ => {
                    return Err("Empty field".to_string());
                }
            }
        }
        _ => {
            return Err("Missing field".to_string());
        }
    };
}

async fn handle_contact_post<'a>(req: Request<Body>, slack: Slack<'a>) -> Result<Response<Body>> {
    // parse body
    let b = req.into_body().try_concat().await?;
    let params = form_urlencoded::parse(&b)
        .into_owned()
        .collect::<HashMap<String, String>>();

    // validate (name, email, message)
    let (name, email, message) = match validate_params(&params) {
        Ok(parsed_params) => parsed_params,
        Err(err) => {
            return Ok(Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .body(Body::from(err))
                .unwrap());
        }
    };

    // send to slack
    let app_env = env::var("APP_ENV").expect("$APP_ENV is missing");
    let channel = format!("contact{}", if app_env == "prd" { "" } else { "-test" });
    let notification_text = format!(":mailbox_with_mail: {}: {}", name, message);
    let title = "*New Message* :wave:";

    let payload = PayloadBuilder::new()
        .channel(&channel)
        .text(&notification_text)
        .blocks(vec![
            SectionBuilder::new(title)
                .fields(vec!["*Name*", "*Email*", &name, &email])
                .build()
                .unwrap()
                .into(),
            SectionBuilder::new(&message[..]).build().unwrap().into(),
        ])
        .build()
        .unwrap();

    let res = match slack
        .payload(payload)
        .send("https://slack.com/api/chat.postMessage")
        .await
    {
        Ok(res) => res,
        Err(_) => {
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to send contact"))
                .unwrap());
        }
    };

    println!("{:#?}", res);

    // return ok
    let res = match params.get("to") {
        Some(to) => Response::builder()
            .status(302)
            .header("Location", to)
            .body(Body::from(""))
            .unwrap(),
        None => Response::builder()
            .status(200)
            .body(Body::from("success"))
            .unwrap(),
    };
    Ok(res)
}

async fn router<'a>(req: Request<Body>, slack: Slack<'a>) -> Result<Response<Body>> {
    println!("{} {}", req.method(), req.uri());
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/contact") => handle_contact_post(req, slack).await,
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(NOTFOUND.into())
            .unwrap()),
    }
}

#[tokio::main]
pub async fn main() -> Result<()> {
    openssl_probe::init_ssl_cert_env_vars();
    pretty_env_logger::init();

    let mut port: u16 = 8080;
    match env::var("PORT") {
        Ok(p) => {
            match p.parse::<u16>() {
                Ok(n) => {
                    port = n;
                }
                Err(_e) => {}
            };
        }
        Err(_e) => {}
    };
    let addr = ([0, 0, 0, 0], port).into();

    let slack_api_token = env::var("SLACK_API_TOKEN").expect("$SLACK_API_TOKEN is missing");
    let slack = Slack::new(slack_api_token);

    let new_service = make_service_fn(move |_| {
        let slack = slack.clone();
        async { Ok::<_, GenericError>(service_fn(move |req| router(req, slack.to_owned()))) }
    });

    let server = Server::bind(&addr).serve(new_service);
    println!("Listening on http://{}", addr);
    server.await?;

    Ok(())
}
