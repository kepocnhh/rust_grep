use crate::config::Config;

fn get_config_by_file(file_path: &String) -> Result<Config, String> {
    if file_path.is_empty() {
        return Err("File path is empty!".to_owned())
    }
    let content = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file by path \"{file_path}\"!");
    return Ok(Config::File { content })
}

pub fn parse_args(args: &[String]) -> Result<Config, String> {
    let len = args.len();

    if len == 1 {
        if args[0] == "--help" {
            return Ok(Config::Help);
        }
    }

    if len == 2 {
        let flag = args[0].as_str();
        return match flag {
            "-f" => return get_config_by_file(&args[1]),
            _ => Err(format!("Flag \"{flag}\" is not supported!"))
        }
    }

    todo!()
}
