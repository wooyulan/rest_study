use std::{env, fs,process, error::Error};

pub fn test_file() {
    test_args();
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }

    fn build(args: &[String]) -> Result<Config, &'stati str> {
        if args.len() < 3 {
            // 'static 作用此error的生命周期
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

// cargo run -- searchstring example-filename.txt
fn test_args() {
    let args: Vec<String> = env::args().collect();

    //   let config = Config::new(&args);

    // 对 build 返回的 `Result` 进行处理
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);

}

fn run(config:Config) -> Result<(),Box<dyn Error>>{
    // 读取文件的内容
    let contents = fs::read_to_string(config.file_path)?;
    println!("文件内容为:\n{contents}");
    Ok(())
}