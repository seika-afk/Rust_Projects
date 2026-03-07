use reqwest::header::USER_AGENT;
use serde::Deserialize;


#[derive(Deserialize,Debug)]
struct User{
    login : String ,
    id: u32 ,
}



#[tokio::main]
async fn main()-> Result<(),Box<dyn std::error::Error>> {

    let request_url=format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
    owner="seika-afk",
    repo="Texin"
    );
    println!("{}", request_url);
    let client= reqwest::Client::new();
    let response= client
                    .get (&request_url)
                    .header(USER_AGENT,"charmander")
                    .send()
                    .await?;
    let users :Vec<User> = response.json().await?;

    println!("{:?}",users);
    Ok(())
}
