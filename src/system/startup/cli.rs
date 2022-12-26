use std::net::IpAddr;
//use std::path::PathBuf;
use argparse::{ArgumentParser, Store};


pub struct RunOptions {
    pub port: u16,
    pub host: IpAddr,
    pub db_host: IpAddr,
    pub db_port: u16,
    pub db_name: String,
    //pub home_dir: PathBuf,
}

impl Default for RunOptions {
    fn default() -> Self {
        return Self {
            port: 8000,
            host: IpAddr::from([0,0,0,0]),
            db_host: IpAddr::from([0,0,0,0]),
            db_port: 27017,
            db_name: String::from("mazzdb"),
            //home_dir: PathBuf,
        }
    }
}

pub fn parse_options() -> RunOptions {
    let mut run_options = RunOptions::default();
    {
        let mut parse = ArgumentParser::new();
        parse.set_description("Mazz - bug tracker backend");
        parse.refer(&mut run_options.host).add_option(&["--host"], Store, "Listen host");
        parse.refer(&mut run_options.port).add_option(&["--port"], Store, "Listen port");
        parse.refer(&mut run_options.db_host).add_option(&["--db-host"], Store, "Database host");
        parse.refer(&mut run_options.db_port).add_option(&["--db-port"], Store, "Database port");
        parse.refer(&mut run_options.db_name).add_option(&["--db-name"], Store, "Database name");
        parse.parse_args_or_exit();
    }

    run_options
}