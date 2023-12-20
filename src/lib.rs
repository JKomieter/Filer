use file_handlers::help;

mod file_handlers;


pub struct Config {
    pub cmd: String,
    pub filename: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            help();
        } 

        let cmd = args[1].clone() ;
        let filename = args[2].clone();

        Ok(
            Config { cmd, filename }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_build() {
        let args = vec![
            "filer".to_string(),
            "create".to_string(),
            "test".to_string(),
        ];

        let config = Config::build(&args).unwrap();

        assert_eq!(config.cmd, "create");
        assert_eq!(config.filename, "test");
    }

    #[test]
    fn test_create_file_cmd() {
        let filename = "test";

        let result = file_handlers::create_file(&filename);

        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_delete_file_cmd() {
        let filename = "test";

        let result = file_handlers::delete_file(&filename);

        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_cmp_file_cmd() {
        let filename = "test";

        let result = file_handlers::compare_versions(&filename);

        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_create_file_version_cmd() {
        let filename = "test";

        let result = file_handlers::create_file_version(&filename);

        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_read_file_contents() {
        let filename = "test";

        let result = file_handlers::read_file_contents(&filename);

        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_compare_versions() {
        let filename = "poem";

        let result = file_handlers::compare_versions(&filename);

        assert_eq!(result.is_ok(), true);
    }
}


