extern crate exonum;
extern crate exonum_configuration;
extern crate node;
extern crate exonum_time;

use exonum::helpers::{self, fabric::NodeBuilder};
use exonum_configuration as configuration;
use node as cryptocurrency;
use std::env;
use exonum_time::TimeServiceFactory;

fn main() {
    env::set_var("RUST_LOG", "info");
    exonum::crypto::init();
    helpers::init_logger().unwrap();

    let node = NodeBuilder::new()
        .with_service(Box::new(configuration::ServiceFactory))
        .with_service(Box::new(cryptocurrency::ServiceFactory))
        .with_service(Box::new(TimeServiceFactory));
    node.run();
}
