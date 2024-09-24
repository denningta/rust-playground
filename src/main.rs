use reqwest::blocking::{Client, Response};
use reqwest::Error;
mod get_ntlm;

fn get(username: &str, password: Option<String>, url: &str) -> Result<Response, Error> {
    let client = Client::new();

    let response = client.get(url).basic_auth(username, password).send()?;

    Ok(response)
}

fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let url = "https://authenticationtest.com/HTTPAuth/";
    let username = "user";
    let password = "pass";

    // let response = get(username, password, url)?;

    let response = get_ntlm::get_ntlm(username, password, url);

    println!("{:?}", response);

    Ok(())
}
