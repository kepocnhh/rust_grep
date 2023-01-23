use crate::entity::Config;

fn get_config_by_file(file_path: &String) -> Result<Config, String> {
    if file_path.is_empty() {
        return Err("File path is empty!".to_owned())
    }
    return match std::fs::read_to_string(file_path) {
        Ok(_) => Ok(Config::File),
        Err(_) => Err(format!("Should have been able to read the file by path \"{file_path}\"!"))
    }
}

pub fn parse_args(args: &[String]) -> Result<Config, String> {
    let len = args.len();

    if len == 0 {
        return Err("No args!".to_owned())
    }

    if len == 1 {
        let flag = args[0].as_str();
        return match flag {
            it if Config::Help.get_flags().contains(&it) => Ok(Config::Help),
            _ => Err(format!("Flag \"{flag}\" is not supported!"))
        }
    }

    if len == 2 {
        let flag = args[0].as_str();
        return match flag {
            it if Config::File.get_flags().contains(&it) => return get_config_by_file(&args[1]),
            _ => Err(format!("Flag \"{flag}\" is not supported!"))
        }
    }

    return Err("Too many args!".to_owned());
}
