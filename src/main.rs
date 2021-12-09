mod error;
pub use error::Error;
mod model;
mod subdomains;
mod common_ports;
mod ports;

use model::Subdomain;
use reqwest::{blocking::Client, redirect};
use rayon::prelude::*;
use std::{env, time::Duration};

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::CliUsage.into())
    }

    let target = args[1].as_str();

    let http_timeout = Duration::from_secs(5);
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(http_timeout)
        .build()?;

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build()
        .unwrap();

    pool.install(|| {
        let scan_result: Vec<Subdomain> = subdomains::enumerate(&http_client, target)
            .unwrap()
            .into_par_iter()
            .map(ports::scan_ports)
            .collect();

        scan_result
            .into_iter()
            .for_each(|subdomain| {
                println!("domain: {}", &subdomain.domain);
                subdomain.open_ports
                    .into_iter()
                    .for_each(|port| println!("port: {}", port.port));

                println!();
            });
        });

    Ok(())
}
