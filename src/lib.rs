extern crate rand;

// for logging (debug mostly, switched at compile time in cargo.toml)
extern crate log;
use env_logger::{Builder};

#[macro_use]
extern crate lazy_static;


pub mod hnsw;
pub mod dist;
pub mod annhdf5;
pub mod hnswio;
pub mod test;
pub mod prelude;
pub mod api;
pub mod libext;

lazy_static! {
    #[allow(dead_code)]
    static ref LOG: u64 = {
        let res = init_log();
        res
    };
}

// install a logger facility
fn init_log() -> u64 {
    Builder::from_default_env().init();
    println!("\n ************** initializing logger *****************\n");    
    return 1;
}
