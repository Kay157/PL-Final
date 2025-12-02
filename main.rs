mod tokens;
mod lexer;

fn main() {
    let src = r#"
        let n;
        n = 5;
        print n;
        func factorial(x) [ if x < 2 [ return 1; ] else [ return x * factorial(x - 1); ] ]
    "#;

    let mut lex = lexer::Lexer::new(src);

    loop {
        let tok = lex.next_token();
        println!("{:?}", tok.code);
        if matches!(tok.code, tokens::TCode::EOI) {
            break;
        }
    }
}
