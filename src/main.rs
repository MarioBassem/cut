use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error, Read},
};

#[derive(Parser, Debug)]
#[clap(name ="cut", author, about, long_about = None)]
struct Params {
    #[clap(short = 'd', default_value_t = String::from(" "), help = "field delimiter")]
    delimiter: String,

    #[clap(short = 'f', required = true,  num_args = 1.., value_delimiter = ',', help = "field numbers")]
    fields: Vec<usize>,

    #[clap(required = true, help = "file path. use '-' for stdin")]
    path: String,
}

fn main() {
    let mut params = Params::parse();

    if let Err(e) = app(&mut params) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn app(params: &mut Params) -> Result<(), Error> {
    let mut reader: BufReader<Box<dyn Read>>;
    if params.path == "-" {
        // use stdin
        reader = BufReader::new(Box::new(io::stdin()));
    } else {
        // open file
        let fl = File::open(params.path.as_str())?;

        reader = BufReader::new(Box::new(fl));
    }

    params.fields.sort();

    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line)?;
        if len == 0 {
            break;
        }

        let res = cut(line.as_str(), &params.delimiter, &params.fields);
        println!("{}", res.join(&params.delimiter));
    }

    Ok(())
}

pub fn cut<'a>(line: &'a str, delimiter: &str, fields: &Vec<usize>) -> Vec<&'a str> {
    let split_line: Vec<&str> = line.trim_end_matches('\n').split(delimiter).collect();
    let mut ret = Vec::new();

    for field in fields {
        if *field < 1 {
            continue;
        }

        if *field > split_line.len() {
            break;
        }

        ret.push(split_line[*field - 1]);
    }

    ret
}
