use vergen::{generate_cargo_keys, ConstantsFlags};

fn main() {
    generate_cargo_keys(ConstantsFlags::SHA_SHORT).expect("Unable to generate the cargo keys!");
}
