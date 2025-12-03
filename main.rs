mod tokens;
mod lexer;
mod parser;
mod descent_parser;

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

    let tests = vec![
        (
            "Test 1: Minimal function",
            r#"
            func main() [
                print 1;
            ]
            "#
        ),
        (
            "Test 2: Simple factorial",
            r#"
            func factorial(x) [
                if x < 2 [
                    return 1;
                ]
                else [
                    return x;
                ]
            ]
            "#
        ),
        (
            "Test 3: Recursive factorial",
            r#"
            func factorial(x) [
                if x < 2 [
                    return 1;
                ]
                else [
                    return x * factorial(x - 1);
                ]
            ]
            "#
        ),
        (
            "Test 4: Multiple functions",
            r#"
            func a() [
                let x;
                x = 10;
                print x;
            ]

            func b(y) [
                while y > 0 [
                    print y;
                    y = y - 1;
                ]
            ]
            "#
        ),
    ];

    for (label, src) in tests {
        println!("===============================");
        println!("{}", label);
        println!("Source:\n{}\n", src);

        let lexer = lexer::Lexer::new(src);
        let mut parser = parser::Parser::new(lexer);

        parser.parse();

        println!("Parser finished OK.\n");
    }
}
