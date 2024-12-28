
pub mod scanner;
pub mod printer;
pub mod parser;
pub mod tokens;

pub fn run(file_contents: String, command_type: String) -> i32 {

    // Scan
    let mut scanner: scanner::Scanner = scanner::Scanner::new(file_contents);
    let tokens: Vec<tokens::Token> = match scanner.scan() {
        Ok(tokens) => tokens,
        Err(_) => return 65,
    };

    // Parse
    let mut parser: parser::Parser = parser::Parser::new();
    let expression: String = match parser.parse(tokens.clone()) {
        Ok(parsed_expression) => parsed_expression,
        Err(_) => return 65,
    };
    
    // Print
    let printer = printer::Printer::new(command_type);
    match printer.print(tokens.clone(), expression) {
        Ok(_) => (),
        Err(_) => return 65,
    }

    return 0;

}