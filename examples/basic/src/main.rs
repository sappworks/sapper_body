#![feature(question_mark)]
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate sapper;
extern crate sapper_body_params;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_json;

use sapper::{SApp, SAppWrapper, Request, Response, Result};



mod biz;
use biz::Biz;


#[derive(Clone)]
struct MyApp;
// must impl it
// total entry and exitice
impl SAppWrapper for MyApp {
    fn before(&self, req: &mut Request) -> Result<()> {
        println!("{}", "in SAppWrapper before.");
        sapper_body_params::process(req)?;
        
        Ok(())
    }
    
    fn after(&self, req: &Request, res: &mut Response) -> Result<()> {
        println!("{}", "in SAppWrapper after.");
        
        Ok(())
    }
}



pub fn main() {
    env_logger::init().unwrap();
    
    let mut sapp = SApp::new();
    sapp.address("127.0.0.1")
        .port(1337)
        .with_wrapper(MyApp)
        .add_module(Biz);
    
    println!("Listening on http://127.0.0.1:1337");
    sapp.run();
    
}
