mod generator;

fn main() {
    if let Err(e) = generator::gen_password() {
        eprintln!("Error: {}", e);
    }
}

