use regex::Regex;
use crossterm::{cursor::MoveToRow, execute, terminal::{Clear, ClearType}};
use std::io::{self, stdout, Read};

pub enum Menu {
    Ownership,
    Structs,
    Enums,
    Reliability,
    Quit,
    Invalid,
}

pub fn advance() {
    let _ = io::stdin().read(&mut [0u8]).unwrap();
    let _ = io::stdin().read(&mut [0u8]).unwrap();
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    execute!(stdout(), MoveToRow(0)).unwrap();
}

pub fn match_substring(target: &str, input: &str) -> bool {
    for i in 0..=target.len() {
        for j in i+1..=target.len() {
            let substring = &target[i..j];
            let pattern = format!(r"(?i)^{}$", substring);
            let regex = Regex::new(&pattern).unwrap();
            if regex.is_match(&input) {
                return true
            }
        }
    }
    false
}

pub fn get_input(message: &str) -> Menu {
    println!("{}", message);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input Failed");

    if match_substring("ownership", input.trim()) {
        //println!("{} maps to \x1b[36mMenu::Ownership\x1b[0m enum variant", input.trim());
        Menu::Ownership
    } else if match_substring("structs", input.trim()) {
        //println!("{} maps to \x1b[36mMenu::Structs\x1b[0m enum variant", input.trim());
        Menu::Structs
    } else if match_substring("enums", input.trim()) {
        //println!("{} maps to \x1b[36mMenu::Enums\x1b[0m enum variant", input.trim());
        Menu::Enums
    } else if match_substring("reliability", input.trim()) {
        //println!("{} maps to \x1b[36mMenu::Reliability\x1b[0m enum variant", input.trim());
        Menu::Reliability
    } else if match_substring("quit", input.trim()) {
        //println!("{} maps to \x1b[36mMenu::Quit\x1b[0m enum variant", input.trim());
        Menu::Quit
    } else {
        //println!("{} maps to \x1b[36mMenu::Invalid\x1b[0m enum variant", input.trim());
        Menu::Invalid
    }
}

pub fn demonstrate_ownership() {
    let s1 = String::from("Hello");
    println!("\x1b[32ms1 \x1b[0minitialized as a constant that allocates and points towards the value '{}' in memory", s1);
    let mut s2 = s1;
    println!("\x1b[32ms2 \x1b[0minitialized as a mut with 'let mut s2 = s1'. Rust's borrow checker transfers ownership of the value '{}' in memory to s2\nRust drops s1 from scope", s2);
    println!("  Any call to s1 like println!(\"{{}}\", s1) would return a compile-time error");
    s2.push_str(", world!");
    println!("s2 is mutable, so we can edit the value in memory that it points to");
    println!("s2 = '{}'\n", s2);

    advance();

    println!("Ownership changes when passing arguments. Here we have two functions:");
    println!("  pub fn foo(s: String) {{}}");
    println!("             ^^^^^^^^^  This function doesn't do anything, but takes ownership");
    println!("  pub fn bar(s: String) -> String {{s}}");
    println!("                                  ^^^  This function just returns ownership");

    println!("\nThe code snippet");
    println!("  let s = String::from(\"hello\");");
    println!("  foo(s);           + foo(s: String) {{");
    println!("     ^^^ foo takes  |   # the scope of s ends here");
    println!("         ownership  + }}");
    println!("  println!(\"{{}}\", s);");
    println!("                 ^ This value was just dropped because its scope ended");
    println!("                   This call is invalid and will throw a compile error");
    println!("The code snippet");
    println!("  let s = String::from(\"hello\");");
    println!("  let s = bar(s);           + bar(s: String) -> String {{");
    println!("             ^^^ bar takes  |   s <- This is a return that extends scope");
    println!("                 ownership  | }}");
    println!("  println!(\"{{}}\", s);        +");
    println!("                 ^ s is still in scope, so this is valid");

    advance();

    println!("\nRust always passes arguments by value, but you can pass references directly:");
    println!("  fn reader(r: &String) {{}}");
    println!("               ^^^^^^^ This is an immutable reference but can accept");
    println!("                       either immutable or mutable references when passed");
    println!("  fn mutator(r: &mut String) {{}}");
    println!("                ^^^^ This is a mutable reference and will only");
    println!("                     accept other mutable references when passed");

    println!("\nbar() can be written as:");
    println!("  pub fn bar(s: &String){{}}");
    println!("                ^^^^^^^ It never takes ownership, so the code can be");
    println!("  let s = String::from(\"hello\");  +");
    println!("  bar(&s);                        | the scope of s ends at its last call");
    println!("  println!(\"{{}}\", s);              +");

    advance();


    println!("Unlike pointers, references are limited in quantity and type. For any given variable, there can only exist 1 mutable reference, and never at the same time as an immutable reference. This prevents developers from writing race conditions.");
    println!("\nThe code snippet:");
    println!("  let mut s = String::from(\"hello\");");
    println!("  let ref1 = &s;");
    println!("  let ref2 = &s;");
    println!("  let ref3 = &mut s;");
    println!("is invalid. Why?");

    println!("ref1 and ref2 share a scope that has to end before ref3 is initialized");
    println!("This scope is determined at compile time, so...");
    println!("  let mut s = String::from(\"hello\");");
    println!("  let ref1 = &s;                   + begin ref1 scope");
    println!("  let ref2 = &s;                   | begin ref2 scope");
    println!("  println!(\"{{}}, {{}}\", ref1, ref2);  + end ref1, ref2 scope");
    println!("  let ref3 = &mut s;");
    println!("             ^^^^^^ letting this be created");

    println!("\nBy contrast...");
    println!("  let mut s = String::from(\"hello\");");
    println!("  let ref1 = &s;                   + begin ref1 scope");
    println!("  let ref2 = &s;                   | begin ref2 scope");
    println!("  let ref3 = &mut s;               |");
    println!("             ^^^^^^ can't be done  |");
    println!("                    ref1 and ref2  |");
    println!("                    are in scope   |");
    println!("  println!(\"{{}}, {{}}\", ref1, ref2);  + end ref1, ref2 scope");

    advance();

    println!("\nDangling pointers can't be made either. The function:\n");
    println!("  pub fn make_null_ptr() -> &String {{");
    println!("    let s = String::from(\"hello\")");
    println!("    &s");
    println!("  }}");
    println!("\nIs invalid and will throw an error at compile time. Why?");

    advance();

    println!("\nWith error annotation:");
    println!("  pub fn make_null_ptr() -> &String {{");
    println!("                            ^^^^^^^ Attempting to return a");
    println!("                                    reference is fine");
    println!("    let s = String::from(\"hello\")");
    println!("            ^^^^^^^^^^^^^^^^^^^^^^^ This data only has scope");
    println!("                                    inside this function");
    println!("    &s");
    println!("    ^^ If this is returned, it would be a null pointer");
    println!("  }}");

    advance();

    println!("\nThe solution?\n");
    println!("  pub fn no_null_ptr() -> String {{");
    println!("    let s = String::from(\"hello\")");
    println!("    s");
    println!("  }}");
    println!("\nJust create and return a String instead.");
    
    advance();
}

pub fn demonstrate_structs() {

    trait semiaquatic {}

    // demo of composition

}

pub fn demonstrate_enums() {

    // Demo of conway's game of life

}

pub fn demonstrate_reliability() {

    // Demo of how results and options work

}

fn main() {
    loop {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
        execute!(stdout(), MoveToRow(0)).unwrap();
        println!("Learn about Rust's strengths with annotated code examples");
        println!("    [Ownership]");
        println!("    [Structs]");
        println!("    [Enums]");
        println!("    [Reliability]");
        println!("    [Quit]");
        let input: Menu = get_input("Take your pick:");
        match input {
            Menu::Ownership => {
                println!("\x1b[36mMenu::Ownership \x1b[0mpattern recognized\n");
                demonstrate_ownership();
            },
            Menu::Structs => {
                println!("\x1b[36mMenu::Structs \x1b[0mpattern recognized\n");
                demonstrate_structs();
            },
            Menu::Enums => {
                println!("\x1b[36mMenu::Enums \x1b[0mpattern recognized\n");
                demonstrate_enums();
            },
            Menu::Reliability => {
                println!("\x1b[36mMenu::Reliability \x1b[0mpattern recognized\n");
                demonstrate_reliability();
            },
            Menu::Quit => {
                println!("\x1b[36mMenu::Quit \x1b[0mpattern recognized\n");
                break
            },
            Menu::Invalid => println!("\x1b[36mMenu::Invalid \x1b[0mpattern recognized\n\n"),
        }
    }
}
