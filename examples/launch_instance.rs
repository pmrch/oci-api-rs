use oci_api_rs::prelude::instance::{AvailabilityConfig, CreateVnicDetails, InstanceDetails, Metadata, ShapeConfig, SourceDetails};
use oci_api_rs::prelude::net::{Client, Request, Response};
use oci_api_rs::prelude::{InstanceDetailsBuilder, Result};
use oci_api_rs::{Secrets, create_signed_request, setup_credentials_from_env};

const OCPUS: u16 = 2;
const MEMORY_GB: u16 = 12;
const REGION: &str = "eu-frankfurt-1";
const AVAILABILITY_DOMAIN: &str = "fZvm:EU-FRANKFURT-1-AD-1";

#[tokio::main]
async fn main() -> Result<()> {
    let (ctx, creds) = setup_credentials_from_env().await?;
    let secrets: Secrets = Secrets::from_env()?;

    let client: Client = Client::new();
    let launch_details: InstanceDetails = InstanceDetailsBuilder::default()
        .availability_domain(AVAILABILITY_DOMAIN)
        .compartment_id(&creds.tenancy)
        .display_name("ampere-instance")
        .source_details(SourceDetails::image(secrets.image_id()))
        .subnet_id(secrets.subnet_id())
        .shape("VM.Standard.A1.Flex")
        .shape_config(ShapeConfig::new(OCPUS, MEMORY_GB))
        .create_vnic_details(CreateVnicDetails::new(secrets.subnet_id()))
        .metadata(Metadata::new(secrets.ssh_key()))
        .is_pv_enabled(true)
        .availability_config(AvailabilityConfig::restore())
        .build()?;

    let launch_details: Vec<u8> = serde_json::to_vec(&launch_details)?;
    let req: Request = create_signed_request(&ctx, launch_details, &creds, REGION).await?;
    let response: Response = client.execute(req).await?;

    println!("{}", response.status());
    println!("{}", response.text().await?);
    Ok(())
}
