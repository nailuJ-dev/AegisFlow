use aegisflow_core::{AuditChain, Capability, DataLabel, Operation, PolicyEngine, ToolRequest};
use anyhow::Context;
use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "aegisflow", version, about = "Evaluate an agent tool request against a deny-by-default policy")]
struct Cli {
    #[arg(long, value_enum)]
    operation: CliOperation,
    #[arg(long, value_enum)]
    label: CliLabel,
    #[arg(long)]
    argument: String,
    #[arg(long, default_value = "workflow-local")]
    subject: String,
    #[arg(long, default_value_t = 60)]
    ttl_seconds: u64,
    #[arg(long)]
    issue_capability: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum CliOperation { FileRead, FileWrite, NetworkGet, NetworkPost, SecretRead }
#[derive(Clone, Copy, Debug, ValueEnum)]
enum CliLabel { Public, Trusted, Untrusted, Secret }

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let operation = match cli.operation {
        CliOperation::FileRead => Operation::FileRead,
        CliOperation::FileWrite => Operation::FileWrite,
        CliOperation::NetworkGet => Operation::NetworkGet,
        CliOperation::NetworkPost => Operation::NetworkPost,
        CliOperation::SecretRead => Operation::SecretRead,
    };
    let label = match cli.label {
        CliLabel::Public => DataLabel::Public,
        CliLabel::Trusted => DataLabel::Trusted,
        CliLabel::Untrusted => DataLabel::Untrusted,
        CliLabel::Secret => DataLabel::Secret,
    };
    let capabilities = if cli.issue_capability {
        vec![Capability::issue(operation, cli.subject.clone(), cli.ttl_seconds).context("capability issuance failed")?]
    } else { Vec::new() };
    let request = ToolRequest::new(&cli.subject, operation, label, cli.argument);
    let decision = PolicyEngine.evaluate(&request, &capabilities);
    let mut audit = AuditChain::default();
    audit.append(&cli.subject, format!("{}:{}", decision.allowed, decision.reason))?;
    println!("{}", serde_json::to_string_pretty(&serde_json::json!({
        "decision": decision,
        "audit_valid": audit.verify(),
        "audit": audit.entries(),
    }))?);
    Ok(())
}
