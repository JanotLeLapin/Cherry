pub mod analyzer;

fn main() {
    let source = std::fs::read_to_string("./main.ch").unwrap();
    let lexer = analyzer::Lexer::new(&source);
    let start = std::time::Instant::now();
    let tokens: Vec<_> = lexer.collect();
    for analyzer::Token { token_type, start, end } in tokens {
        println!("{:?}: {:?} ({}..{})", token_type, &source[start..end], start, end);
    }
    println!("{:?}", std::time::Instant::now() - start);
}
