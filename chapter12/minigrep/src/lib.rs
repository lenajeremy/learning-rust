use std::{env, error::Error};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Config {
        let config = parse_config(args).unwrap_or_else(|err| {
            panic!("problem parsing arguments. err: {}", err);
        });
        config
    }
}

fn parse_config(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
        return Err("not enough arguments");
    }

    let query = args[1].clone();
    let file_path = args[2].clone();
    let case_sensitive = env::var("CASE_SENSITIVE").is_ok();

    Ok(Config {
        query,
        file_path,
        case_sensitive,
    })
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = std::fs::read_to_string(config.file_path)?;

    let results;
    if config.case_sensitive {
        results = search(&config.query, &file_content);
    } else {
        results = search_case_insensitive(&config.query, &file_content)
    }

    if results.is_empty() {
        println!("Search query not found");
    }

    for line in results {
        println!("{line}");
    }
    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res = vec![];

    content.split('\n').for_each(|line| {
        if line.contains(query) {
            res.push(line);
        }
    });

    res
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res = vec![];
    let query = query.to_lowercase();

    content.lines().for_each(|line| {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    });

    res
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_search() {
        let contents = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";
        let lines = search("you", contents);
        let expected = vec![
            "I'm nobody! Who are you?",
            "Are you nobody, too?",
            "They'd banish us, you know.",
            "To tell your name the livelong day",
        ];

        for (index, line) in expected.iter().enumerate() {
            assert_eq!(line, &lines[index]);
        }
    }

    #[test]
    fn test_parse_config() {
        let args = vec![
            "program_name".to_string(),
            "poem.txt".to_string(),
            "you".to_string(),
        ];
        let expected_config = Config {
            case_sensitive: true,
            file_path: "poem.txt".to_string(),
            query: "you".to_string(),
        };

        assert_eq!(expected_config.file_path, args[1]);
        assert_eq!(expected_config.query, args[2]);
    }

    #[should_panic(expected = "problem parsing arguments")]
    #[test]
    fn test_parse_config_with_empty_args() {
        let args: Vec<String> = Vec::new();
        let config = Config::build(&args);
        println!("{}:{}", config.query, config.file_path);
    }

    #[test]
    fn text_search_case_insensitive() {
        let query = "BODY";
        let contents = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";
        let res = search_case_insensitive(query, contents);
        let expected_res = vec![
            "I'm nobody! Who are you?",
            "Are you nobody, too?",
            "How dreary to be somebody!",
        ];

        println!("res={:?}\nexpected_res={:?}", res, expected_res);

        assert_eq!(res.len(), expected_res.len());

        for (index, line) in res.iter().enumerate() {
            assert_eq!(line, &expected_res[index]);
        }
    }
}
