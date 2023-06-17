use serde::Deserialize;
use std::fs;

fn main() {
    KongJSBuilder::build();
}

#[derive(Deserialize)]
struct Config {
    src_dir: String,
    out_js_file: String,
    accounts: Option<bool>,
    login: Option<bool>,
    properties: Option<bool>,
}

impl Config {
    pub fn read() -> Config {
        let arg = std::env::args().nth(1);
        match arg {
            Some(a) => {
                let toml_str = fs::read_to_string(a).unwrap();
                let config: Config = toml::from_str(&toml_str).unwrap();
                config
            }
            None => panic!("Path to config file is not provided!"),
        }
    }
}
struct KongJSBuilder;

impl KongJSBuilder {
    pub fn build() {
        let config = Config::read();
        let mut output_src = KongJSBuilder::copy_mainjs(&config.src_dir);

        if let Some(accounts) = config.accounts {
            if accounts {
                let accountsjs = KongJSBuilder::copy_accountsjs(&config.src_dir);
                output_src = format!("{output_src}{accountsjs}");
            }
        }

        if let Some(login) = config.login {
            if login {
                let loginjs = KongJSBuilder::copy_loginjs(&config.src_dir);
                output_src = format!("{output_src}{loginjs}");
            }
        }

        if let Some(properties) = config.properties {
            if properties {
                let propertiesjs = KongJSBuilder::copy_propertiesjs(&config.src_dir);
                output_src = format!("{output_src}{propertiesjs}");
            }
        }

        KongJSBuilder::save_src(&output_src, &config.out_js_file);
    }

    fn copy_mainjs(src_path: &str) -> String {
        let src_path = format!("{src_path}/main.js");
        fs::read_to_string(src_path).unwrap()
    }

    fn copy_accountsjs(src_path: &str) -> String {
        let src_path = format!("{src_path}/accounts.js");
        fs::read_to_string(src_path).unwrap()
    }

    fn copy_loginjs(src_path: &str) -> String {
        let src_path = format!("{src_path}/login.js");
        fs::read_to_string(src_path).unwrap()
    }

    fn copy_propertiesjs(src_path: &str) -> String {
        let src_path = format!("{src_path}/properties.js");
        fs::read_to_string(src_path).unwrap()
    }

    fn save_src(src: &str, out_js_file: &str) {
        fs::write(out_js_file, src.as_bytes()).unwrap();
    }
}
