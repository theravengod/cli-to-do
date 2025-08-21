use std::env;
use todo_lib as lib;

fn main() {
    lib::run_mode(&env::args().collect())
}
