
mod tokens;
mod lexer;
mod parser;
mod descent_parser;
mod mtree;
mod analyzer;
mod evaluator;

use std::env;
use std::fs;

fn main() {
    // user input should look like:
    // cargo run [command] [file]


    // Access terminal input from user
    let args: Vec<String> = env::args().collect();
    let terminal_input = args.clone();

    // Check user input
    let function = match args.get(1) {
        Some(arg) => arg,
        None => {
            println!("No command provided. Use 'help' to see available commands.");
            return;
        }
    };

    // Call the corresponding function for the inputted command
    match function.as_str() {
        "help" => help(terminal_input),
        "list" => list_commands(),
        "tokenize" => configure_lexer(terminal_input),
        "parse" => parse_file(terminal_input),
        "execute" => execute(terminal_input),
        _ => println!("Unknown command: {function}")
    }
}

// The help command displays a help message to clarify each command
fn help(args: Vec<String>) {
    // A specific command specified
    if let Some(command) = args.get(2) {
        match command.as_str() {
            "[help]" => {
                println!("help,                         Display the help message for all commands.");
                println!("help [command]                Display the help message for the specified command.");
            }
            "[list]" => {
                println!("list                         Display the list of commands.");
            }
            "[tokenize]" => {
                println!("tokenize                      Perform lexical analysis on a given file.");
            }
            "[parse]" => {
                println!("parse                         Parse a given input file and print the resulting parse tree.");
            }
            "[execute]" => {
                println!("execute                       Execute a given input file and print the tree and the result of the program.");
            }
            _ => println!("Unknown command: {command}"),
        }
    }
    // No specific command specified
    else {
        println!("help                Display the help message for all commands.");
        println!("help [command]      Display the help message for the specified command.");
        println!("list                Display the list of supported commands.");
        println!("tokenize            Display the lexical analysis on a given file.");
        println!("parse               Display the parsed tree given an input file.");
        println!("execute             Display the executed code and the parsed tree of a given input file.");
    }
}

// The list command prints the list of available commands
fn list_commands() {
    println!("Available Commands: ");
    println!("help, ");
    println!("list, ");
    println!("tokenize, ");
    println!("parse, ");
    println!("execute ");
}

// The configure lexer method is called for the tokenize command to create a new lexer from the file input for analysis
fn configure_lexer(args: Vec<String>) {
    let file_path = match args.get(2) {
        Some(path) => path,
        None => {
            println!("No file specified.");
            return;
        }
    };

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let mut lex = lexer::Lexer::new(&*contents);
    loop {
        let tok = lex.next_token();
        println!("{:?}", tok.code);
        if matches!(tok.code, tokens::TCode::EOI) {
            break;
        }
    }
}

fn parse_file(args: Vec<String>) {
    let file_path = match args.get(2) {
        Some(path) => path,
        None => {
            println!("No file specified.");
            return;
        }
    };

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let lexer = lexer::Lexer::new(&*contents);
    let mut parser = parser::Parser::new(lexer);
    let ast = parser.parse();
    println!("--- AST (MTree) ---");
    ast.borrow().print();
}

pub fn execute(args: Vec<String>) {
    let file_path = match args.get(2) {
        Some(path) => path,
        None => {
            println!("No file specified.");
            return;
        }
    };
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let lexer = lexer::Lexer::new(&*contents);
    let mut parser = parser::Parser::new(lexer);
    let ast = parser.parse();
    println!("--- ANALYZING ---");
    analyzer::analyze(ast.clone());

    println!("--- RUNNING PROGRAM ---");
    let mut runtime = evaluator::Runtime::new();
    runtime.run_program(ast.clone());

    println!("--- DONE ---\n");
}