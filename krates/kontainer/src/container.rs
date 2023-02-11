use serde::Deserialize;

#[derive(Deserialize)]
pub struct Container {
    name: String,
    distribution: String,
    release: String,
    architecture: String,
}

impl Container {
    /// Create a new container instance
    pub fn create(&self) {
        std::process::Command::new("/usr/bin/lxc-create")
            .arg("-t")
            .arg("/usr/share/lxc/templates/lxc-download")
            .arg("-n")
            .arg(&self.name)
            .arg("--")
            .arg("-d")
            .arg(&self.distribution)
            .arg("-r")
            .arg(&self.release)
            .arg("-a")
            .arg(&self.architecture)
            .status()
            .expect("Failed to create container");

        println!("Container {} created", &self.name,);
    }
}
