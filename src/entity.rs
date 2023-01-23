pub enum Config {
    Help,
    File
}

impl Config {
    pub fn get_flags(&self) -> Vec<&str> {
        return match self {
            Config::Help => vec!["-h", "--help"],
            Config::File => vec!["-f", "--file"]
        }
    }
}
