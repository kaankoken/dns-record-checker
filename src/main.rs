use clap::{builder, Parser};
use color_eyre::{eyre::Context, Result};
use std::path::Path;
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};

#[derive(Debug, Parser)]
#[command(name = "dns-record")]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// The xlsx file to read
    #[arg(short, long, required = true, value_parser = builder::NonEmptyStringValueParser::new())]
    file: String,

    /// The sheet to read
    #[arg(short, long, value_parser = builder::NonEmptyStringValueParser::new(), default_value = "Sheet1")]
    sheet: String,
}

fn check_files_exist<P: AsRef<Path>>(file_path: P) -> Result<()> {
    let is_exist = file_path
        .as_ref()
        .try_exists()
        .wrap_err("file permission related issue")?;

    let is_file = file_path.as_ref().is_file();

    if !is_exist {
        return Err(color_eyre::eyre::eyre!("file does not exist"));
    }

    if !is_file {
        return Err(color_eyre::eyre::eyre!("given path is not a file"));
    }

    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Arguments::parse();

    check_files_exist(&args.file)?;

    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())
        .wrap_err("failed to create resolver")?;

    resolver
        .lookup_ip("google.com")
        .wrap_err("failed to lookup ip")?;

    println!("{:?}", args);
    Ok(())
}
