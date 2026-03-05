use csv;
use std::error::Error;
use std::io;
use std::env::args;
use std::io::Write;
fn main()-> Result<(),Box<dyn Error>>{


    //taking input - as file
    let args : Vec<String> =args().collect();

    if args.len()<2{

        eprint!("Usage: Cmd <filename>");
        return Ok(());


    }
    let p = &args[1];

    //taking input - as number - how many rows to see
    let mut input=String::new();
    print!("> ");
    io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

    //csv stuff
    let mut reader= csv::Reader::from_path(p)?;
    let rinput: usize = input.trim().parse()?;
    for result  in reader.records().take(rinput){
        let record= result?;

        for i in 0..record.len(){
 print!("{}  ",record.get(i).unwrap_or(""));


    }
       println!("");

    }
Ok(())



}
