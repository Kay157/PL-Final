
mod tokens;
mod lexer;
mod parser;
mod descent_parser;
mod mtree;
mod analyzer;
mod evaluator;

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
        /*
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

        (
            "Test 5: Undeclared variable",
            r#"
                func test(y) [
                    print x;
                    let x;
                    x = 5;
                    print y;
                ]
            "#
        ),
        */
        (
            "Test 6: Final test",
            r#"
            func factorial_recursion(n) [
                if n < 2 [
                    return 1;
                ]
                else [
                    return n * factorial_recursion(n - 1);
                ]
            ]

            func factorial_loop(n) [
                let p;
                p = n;

                while n > 0 [
                    n = n - 1;
                    p = p * n;
                ]
                return p;
            ]

            func main() [
                let n;
                n = 5;
                print factorial_loop(n);
                print factorial_recursion(n);
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

        let ast = parser.parse();

        println!("--- AST (MTree) ---");
        ast.borrow().print();

        // --- NEW: Run analyzer ---
        println!("--- ANALYZING ---");
        analyzer::analyze(ast.clone());

        println!("--- RUNNING PROGRAM ---");
        let mut runtime = evaluator::Runtime::new();
        runtime.run_program(ast.clone());

        println!("--- DONE ---\n");
    }

    // The program will throw an error if a variable is not declared
    // and if there is not a main function for it to run.
    // If it throws an error the rest of the tests won't run, so I
    // commented them out!
    // Test 6 is the example from the top of the final project assignment sheet.
}
