use std::collections::HashMap;
use std::io::{self, Write};
#[derive(Debug, PartialEq)]
enum Check {
    NoVals,
    NoValsGet,
    NoValsDel,
    ValDel,
    ValSet,
}
#[derive(Debug, PartialEq)]
enum Errors {
    NoCommand,
    Nofield,
    InvalidCommand,
}
#[derive(Debug, PartialEq)]
enum Commands {
    SET(String, String),
    GET(String),
    DEL(String),
    EXIT,
}
fn parse(instruction: &str) -> Result<Commands, Errors> {
    let instruction = instruction.trim();
    if instruction.is_empty() {
        return Err(Errors::NoCommand);
    }

    let (cmd, rest) = match instruction.split_once(|c: char| c.is_whitespace()) {
        Some((c, r)) => (c, r.trim()),
        None => (instruction, ""),
    };

    match cmd.to_uppercase().as_str() {
        "SET" => {
            let (key, val) = rest
                .split_once(|c: char| c.is_whitespace())
                .ok_or(Errors::Nofield)?;

            Ok(Commands::SET(key.to_string(), val.trim().to_string()))
        }
        "GET" => {
            if rest.is_empty() {
                return Err(Errors::Nofield);
            }
            Ok(Commands::GET(rest.to_string()))
        }
        "DEL" => {
            if rest.is_empty() {
                return Err(Errors::Nofield);
            }
            Ok(Commands::DEL(rest.to_string()))
        }
        "EXIT" => Ok(Commands::EXIT),
        _ => Err(Errors::InvalidCommand),
    }
}
fn set_val(storage: &mut HashMap<String, String>, key: &str, val: &str) -> Check {
    storage.insert(key.to_string(), val.to_string());
    Check::ValSet
}

fn get_val(store: &HashMap<String, String>, key: &str) -> Result<String, Check> {
    if store.is_empty() {
        return Err(Check::NoVals);
    }
    match store.get(key) {
        Some(k) => Ok(k.to_string()),
        None => Err(Check::NoValsGet),
    }
}

fn del_val(store: &mut HashMap<String, String>, key: &str) -> Result<Check, Check> {
    if store.is_empty() {
        return Err(Check::NoVals);
    }
    match store.remove(key) {
        Some(_) => Ok(Check::ValDel),
        None => Err(Check::NoValsDel),
    }
}

fn print_status(status: Check) {
    match status {
        Check::ValSet => println!("VALUE SET SUCCESSFULL"),
        Check::NoVals => println!("THE MAP IS EMPTY"),
        Check::NoValsGet => println!("NO SUCH KEY EXIST TO GET THE VALUE"),
        Check::NoValsDel => println!("NO SUCH KEY EXIST TO DELETE"),
        Check::ValDel => println!("DELETION SUCCESFULL"),
    }
}

fn main() {
    let mut store: HashMap<String, String> = HashMap::new();
    println!("WELCOME TO IRONLOG");

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut cmd = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to take input");

        match parse(&cmd) {
            Ok(Commands::SET(k, v)) => {
                let status = set_val(&mut store, &k, &v);
                print_status(status);
            }
            Ok(Commands::GET(k)) => match get_val(&store, &k) {
                Ok(val) => println!("{}", val),
                Err(err_status) => print_status(err_status),
            },
            Ok(Commands::DEL(k)) => match del_val(&mut store, &k) {
                Ok(success_status) => print_status(success_status),
                Err(err_status) => print_status(err_status),
            },
            Ok(Commands::EXIT) => {
                break;
            }
            Err(Errors::Nofield) => println!("NOT ENOUGH FIELD EXIST TO EXECUTE COMMAND"),
            Err(Errors::NoCommand) => println!("NO COMMAND ENTERED"),
            Err(Errors::InvalidCommand) => println!("NO SUCH COMMAND EXIST"),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setting_value() {
        let input = "SET name Deep";
        let expected = Ok(Commands::SET("name".to_string(), "Deep".to_string()));
        assert_eq!(parse(input), expected);
    }
    #[test]
    fn set_with_space() {
        let input = "SET name John Spark";
        let expected = Ok(Commands::SET("name".to_string(), "John Spark".to_string()));
        assert_eq!(parse(input), expected);
    }
    #[test]
    fn test_parse_get_and_del() {
        assert_eq!(parse("GET mykey"), Ok(Commands::GET("mykey".to_string())));
        assert_eq!(parse("DEL mykey"), Ok(Commands::DEL("mykey".to_string())));
    }

    #[test]
    fn test_parse_errors() {
        assert_eq!(parse(""), Err(Errors::NoCommand));
        assert_eq!(parse("   "), Err(Errors::NoCommand));
        assert_eq!(parse("SET justakey"), Err(Errors::Nofield));
        assert_eq!(parse("GET"), Err(Errors::Nofield));
        assert_eq!(parse("FAKE_COMMAND mykey"), Err(Errors::InvalidCommand));
    }

    #[test]
    fn test_database_operations() {
        let mut store = HashMap::new();

        assert_eq!(get_val(&store, "name"), Err(Check::NoVals));
        assert_eq!(del_val(&mut store, "name"), Err(Check::NoVals));

        assert_eq!(set_val(&mut store, "name", "Ironman"), Check::ValSet);

        assert_eq!(get_val(&store, "name"), Ok("Ironman".to_string()));

        assert_eq!(get_val(&store, "fake_key"), Err(Check::NoValsGet));
        assert_eq!(del_val(&mut store, "fake_key"), Err(Check::NoValsDel));

        assert_eq!(del_val(&mut store, "name"), Ok(Check::ValDel));

        assert_eq!(get_val(&store, "name"), Err(Check::NoVals));
    }
}
