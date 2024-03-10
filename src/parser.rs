use toml;
use toml::{Value, Table};
use std::fs;
use std::fs::File;

#[derive(Debug)]
pub struct DotsConfig {
    pub repositories: Table,
    pub environment: Table
}

impl DotsConfig {
    pub fn from_table(dots_toml: Table) -> DotsConfig{
        let (env, repo) = dots_toml.into_iter().partition(|(_,v)| is_repository(v));
        DotsConfig{
            repositories: Table::from(env),
            environment: Table::from(repo)
        }
    }
    pub fn from_fp(dots_fp: &str) -> Result<DotsConfig, toml::de::Error> {
        read_toml(dots_fp).map(|v| DotsConfig::from_table(v))
    }
}

pub fn create_toml(fp: &str) -> std::io::Result<File> {
    File::create(fp)
}

pub fn read_toml(fp: &str) -> Result<Table, toml::de::Error> {
    let file = match fs::read_to_string(fp) {
        Ok(string) => string,
        Err(_) => {
            let _ = create_toml(fp);
            "".to_string()
        }
    };

    toml::from_str(&file)
}

fn is_repository(v: &Value) -> bool {
    v.is_table()
}
