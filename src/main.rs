pub mod analyzer;

fn main() {
    let source = std::fs::read_to_string("./main.ch").unwrap();
    let lexer = analyzer::Lexer::new(&source);
    
    let mut parser = analyzer::Parser::new(&source, lexer);
    let res = parser.parse();
    println!("{res:#?}");
}
