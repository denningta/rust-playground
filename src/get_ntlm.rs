use curl::easy::{Auth, Easy};
use std::error::Error;

pub fn get_ntlm(username: &str, password: &str, url: &str) -> Result<(), Box<dyn Error>> {
    let mut easy = Easy::new();

    easy.verbose(true)?;
    easy.url(url).unwrap();

    easy.http_auth(Auth::new().ntlm(true))?;

    easy.username(username)?;
    easy.password(password)?;

    let mut response_data = Vec::new();

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            response_data.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }

    println!("Response body: {:?}", String::from_utf8(response_data)?);

    Ok(())
}
