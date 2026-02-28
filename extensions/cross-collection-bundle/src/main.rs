use shopify_function::prelude::*;
use std::process;

pub mod cart_transform_run;

#[typegen("schema.graphql")]
pub mod schema {
    #[query("src/cart_transform_run.graphql")]
    pub mod cart_transform_run {}
}

fn main() {
    log!("Please invoke a named export.");
    process::abort();
}


// use std::process;
// use shopify_function::prelude::*;

// // This module name must match the target name used in schema::cart_transform_run
// pub mod cart_transform_run;

// #[typegen("./schema.graphql")]
// pub mod schema {
//     // Map the Input query to this module.
//     // Also map the configuration jsonValue scalar to our Rust Configuration type.
//     #[query(
//         "src/cart_transform_run.graphql",
//         custom_scalar_overrides = {
//           "Input.cartTransform.metafield.jsonValue" => super::cart_transform_run::Configuration
//         }
//     )]
//     pub mod cart_transform_run {}
// }

// fn main() {
//     eprintln!("Please invoke a named export (cart_transform_run).");
//     process::exit(1);
// }
