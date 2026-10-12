use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Faltan argumentos. Uso: buscador_texto <texto> <archivo>")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problema al analizar los argumentos: {}", err);
        process::exit(1);
    });

    println!("Buscando '{}' en el archivo {}", config.query, config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Error en la aplicación: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), std::io::Error> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn un_resultado() {
        let query = "seguro";
        let contents = "\
Rust:
seguro, rápido, productivo.
Elige tres.";

        assert_eq!(vec!["seguro, rápido, productivo."], search(query, contents));
    }
}