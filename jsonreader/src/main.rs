use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
struct Paragraph{

    name:String,
}

#[derive(Serialize,Deserialize)]
struct Article{
    article : String,
    author:String,
    paragraph : Vec<Paragraph>


}

fn read_json_typed(raw_json: &str)->Result<Article, serde_json::Error>{


    let parsed:Article = serde_json::from_str(raw_json)?;
        Ok(parsed)


}


fn main()->Result<(), Box<dyn std::error::Error>>{

    let json=r#"
                {
                "article":"How to not be",
                "author" : "article itself",
                "paragraph":[{"name":"ugh i dont know"},{"name": "what do you need"}]
                        

                 }"#;
    
    let parsed:Article = read_json_typed(json)?;
    println!("\n\n The name of the first parargraph is : {} ",parsed.paragraph[0].name);



    Ok(())
}
