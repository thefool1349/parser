use parser::parser::parser;

fn main() {
    let okk = parser();

    if let Err(err) = okk {
        println!("{}", err)
    }
}
