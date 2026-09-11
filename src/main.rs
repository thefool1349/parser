use parser::{parser::parser, parser_v2::parser_v2};

fn main() {
    let okk = parser_v2();

    if let Err(err) = okk {
        println!("{}", err)
    }
}
