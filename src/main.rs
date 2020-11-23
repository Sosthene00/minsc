use minsc::{parse, run, Result};
use std::{env, fs, io};

fn main() -> Result<()> {
    let mut args = env::args();
    let input = args.nth(1).unwrap_or_else(|| "-".into());
    let print_ast = args.next() == Some("--ast".into());

    let mut reader: Box<dyn io::Read> = match &*input {
        "-" => Box::new(io::stdin()),
        _ => Box::new(fs::File::open(input)?),
    };

    let mut code = String::new();
    reader.read_to_string(&mut code)?;

    let ast = parse(&code)?;

    if print_ast {
        println!("{:#?}", ast);
    } else {
        let res = run(ast)?;
        println!("{}\n", res);
        println!("{:#?}", res);
    }

    Ok(())
}
