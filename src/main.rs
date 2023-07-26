mod helper_functions;
mod home_converter;
use std::fs;

#[tokio::main]
async fn main() {
use crate::helper_functions::helper_functions::*;
    use crate::home_converter::home_converter::*;

    home_converter().await;
}
