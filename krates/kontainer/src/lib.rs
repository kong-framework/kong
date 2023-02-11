use crate::container::Container;
use serde::Deserialize;
use std::fs;

pub mod container;

#[derive(Deserialize)]
pub struct Sys {
    /// Linux Containers
    pub containers: Vec<Container>,
}

#[derive(Deserialize)]
pub struct Spin {
    pub sys: Sys,
}

impl Spin {
    /// Read Spin.toml file from path provided as an argument when
    /// the program was started.
    pub fn read_spin_file(spin_file: Option<String>) -> Result<Spin, ()> {
        match spin_file {
            Some(a) => {
                let toml_str = fs::read_to_string(a).unwrap();
                let config: Spin = toml::from_str(&toml_str).unwrap();
                Ok(config)
            }
            None => panic!("Path to config file is not provided!"),
        }
    }

    /// List out containers
    pub fn list_containers() {
        std::process::Command::new("/usr/bin/lxc-ls")
            .status()
            .expect("Failed to list containers");
    }

    /// Start a container in the background
    pub fn start_daemon(name: &str) {
        std::process::Command::new("/usr/bin/lxc-start")
            .arg("-n")
            .arg(name)
            .status()
            .expect("Failed to start daemon container");
    }

    /// Attach daemon container
    pub fn attach_daemon(name: &str) {
        std::process::Command::new("/usr/bin/lxc-attach")
            .arg("--clear-env")
            .arg("-n")
            .arg(name)
            .status()
            .expect("Failed to start container");
    }

    /// Start a container
    pub fn start_container(name: &str) {
        Spin::start_daemon(name);
        Spin::attach_daemon(name);
    }
}
