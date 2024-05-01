#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]

extern crate dotenv;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

use log::{info, debug, error, trace};
use dotenv::dotenv;
use std::env;
use std::process::Command;
use std::fs::File;
use std::io::Write;
use std::process::Output;
use reqwest::Response;
use reqwest::Client;
use reqwest::header;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    dotenv().ok();
    for (key, value) in env::vars() {
        debug!("{}: {}", key, value);
    }

    // Get the environment variables
    let subscription_id: String = match env::var("AZURE_SUBSCRIPTION_ID") {
        Ok(val) => val,
        Err(e) => {
            error!("AZURE_SUBSCRIPTION_ID is not set: {}", e);
            std::process::exit(1);
        }
    };
    let resource_group_name: String = match env::var("AZURE_RESOURCE_GROUP_NAME") {
        Ok(val) => val,
        Err(e) => {
            error!("AZURE_RESOURCE_GROUP_NAME is not set: {}", e);
            std::process::exit(1);
        }
    };
    let vm_name: String = match env::var("AZURE_VM_NAME") {
        Ok(val) => val,
        Err(e) => {
            error!("AZURE_VM_NAME is not set: {}", e);
            std::process::exit(1);
        }
    };
    let api_version: String = match env::var("AZURE_API_VERSION") {
        Ok(val) => val,
        Err(e) => {
            error!("AZURE_API_VERSION is not set: {}", e);
            std::process::exit(1);
        }
    };
    let access_token: String = match env::var("AZURE_ACCESS_TOKEN") {
        Ok(val) => val,
        Err(e) => {
            error!("AZURE_ACCESS  is not set: {}", e);
            std::process::exit(1);
        }
    };

    // https://learn.microsoft.com/en-us/rest/api/compute/virtual-machines/get?view=rest-compute-2024-03-01&tabs=HTTP#get-a-virtual-machine.
    let api_url: String = format!(
        "https://management.azure.com/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/virtualMachines/{vm_name}?api-version={api_version}",
        subscription_id = subscription_id,
        resource_group_name = resource_group_name,
        vm_name = vm_name,
        api_version = api_version,
    );

    info!("Use Azure API URL: {}", api_url);

    //
    // Test reqwest
    //
    // let resp = reqwest::get("https://httpbin.org/ip")
    //     .await?
    //     .json::<HashMap<String, String>>()
    //     .await?;
    // println!("{resp:#?}");
    // Ok(())

    let client: Client = reqwest::Client::new();
    let response: Response = client
        .get(&api_url)
        .bearer_auth(access_token)
        .send()
        .await?;

    debug!("Headers:\n{:#?}", response.headers());
    let json_output:String = response.text().await?;
    debug!("{}", json_output);

    //
    // Hard code
    //
    // Due to the jq-rs cannot use in Macos, we use the Command to run the jq command.
    let path: &str = "/tmp/azure_vm.json";

    let mut output: File = File::create(path)?;
    write!(output, "{}", json_output)?;

    let vm_name: Output= Command::new("jq")
    .arg(".name")
    .arg(path)
    .output()
    .expect("failed to execute process");

    let vm_provisioning_state: Output = Command::new("jq")
        .arg(".properties.provisioningState")
        .arg(path)
        .output()
        .expect("failed to execute process");

    let vm_time_created: Output = Command::new("jq")
        .arg(".properties.timeCreated")
        .arg(path)
        .output()
        .expect("failed to execute process");

    println!(
        ".name: {}.properties.provisioningState: {}.properties.timeCreated: {}",
        String::from_utf8_lossy(&vm_name.stdout),
        String::from_utf8_lossy(&vm_provisioning_state.stdout),
        String::from_utf8_lossy(&vm_time_created.stdout)
    );


    //
    // Check x-ms-request-id
    // https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-ncnbi/565d1b95-30cc-4782-aae5-ba636373f8b6

    // https://learn.microsoft.com/en-us/rest/api/compute/virtual-machines/get?view=rest-compute-2024-03-01&tabs=HTTP#get-a-virtual-machine.
    GET https://management.azure.com/providers/Microsoft.Compute/operations?api-version=2024-03-01
    let check_provisioning_url: String = format!(
        "https://management.azure.com/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/virtualMachines/{vm_name}?api-version={api_version}",
        subscription_id = subscription_id,
        resource_group_name = resource_group_name,
        vm_name = vm_name,
        api_version = api_version,
    );

    info!("Use Azure API URL: {}", check_provisioning_url);

    Ok(())

}
