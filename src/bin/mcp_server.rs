use std::process::Command;
use std::env::var;
use serde::Deserialize;
use schemars::JsonSchema;
use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router, ServiceExt, transport::stdio};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ProposeActionParams {
    description: String,
    tier: u32
}

impl ProposeActionParams {
    pub fn new(description:String, tier:u32)->Self {
        ProposeActionParams { description, tier }
    } 
}

#[derive(Clone)]
pub struct FamilyProofServer;

#[tool_router(server_handler)]
impl FamilyProofServer {
    #[tool(description="Propose a new action to Family Constitution")]
    async fn propose_action(&self, params:Parameters<ProposeActionParams>) ->String{
        let output = Command::new("cargo")
            .args([
                "run",
                "--release",
                "--bin",
                "propose_action",
                &params.0.description,
                &params.0.tier.to_string(),
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("failed to propose action as MCP");
        
        if output.status.success() {
           return String::from(
                format!("action: {:?} is proposed - tier: {:?}",&params.0.description,&params.0.tier.to_string())
            );
        } else {
            eprintln!("cast send failed: {:?}", output.status);
            return output.status.to_string();
        }
    
    }
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()>{
    let service = FamilyProofServer.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}