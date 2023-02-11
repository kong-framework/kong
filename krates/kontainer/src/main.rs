use kontainer::Spin;
use std::env;

fn main() {
    let cmd = env::args().nth(1);

    if let Some(cmd) = cmd {
        match cmd.as_str() {
            "create" | "c" => {
                let f = env::args().nth(2);
                let spin = Spin::read_spin_file(f).unwrap();

                for container in &spin.sys.containers {
                    container.create();
                }
            }
            "list" | "l" => Spin::list_containers(),
            "start" | "s" => {
                if let Some(name) = env::args().nth(2) {
                    Spin::start_container(&name);
                } else {
                    panic!("Invalid container name")
                }
            }
            _ => panic!("Unknown command"),
        }
    }
}
