use std::fs;
use std::thread;
use std::time::Duration;
use serde::{Serialize,Deserialize};
use serde_json;

#[derive(Serialize,Deserialize)]
struct Paragraph{

    name  : String


}

#[derive(Serialize,Deserialize)]

struct Article{
    article:String,
    author:String,
    paragraph:Vec<Paragraph>

}





fn main()->Result<(),Box<dyn std::error::Error>> {

        let article :Article= Article { 
        article:String::from("pokemon") 
        , author: String::from("vks")
        , paragraph: vec![
                Paragraph{
                     name:String::from("first sentence")
                    }
        ,         Paragraph{
                     name:"second sentence".into()
                    }
        

        ]



    };

    let json= serde_json::to_string(&article)?;

    println!("the json is  {}",json);

    println!("saving into demo.json...");
    fs::write("demo.json", json)?;
    println!("..."); 
    thread::sleep(Duration::from_secs(1));
   
    println!("saved into demo.json");


    Ok(())
}
