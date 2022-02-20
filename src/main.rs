use anyhow::Result;
use clap::ArgMatches;
use clap::{Arg, Command};
use std::path::Path;
use std::path::PathBuf;

use crate::plists::Agent;
use crate::plists::PList;

mod plists;

fn main() -> Result<()> {
    let m = args_parser().get_matches();

    match m.subcommand() {
        Some(("list", matches)) => cmd_list_agents(matches),
        None => todo!(),
        _ => todo!()
    }
}

fn args_parser() -> clap::Command<'static> {
    Command::new("install-agent")
        .author("Mordecai <mordecai@malignat.us>")
        .subcommand(Command::new("list"))
        .arg(
            Arg::new("command")
                .long("command")
                .short('c')
                .help("The command to be run. Can be a simple path, or a complex invocation."),
        )
        .arg(
            Arg::new("shell")
                .long("shell")
                .short('s')
                .help("Runs command in a shell"),
        )
}

fn cmd_install_agent(args: &clap::ArgMatches) -> Result<()> {
    let agent_spec = Agent::from_args(args)?;
    let plist = PList::from_agent(&agent_spec)?;
    let filename = agent_spec.label + ".plist";
    let target_path = PathBuf::from("~/Library/LaunchAgents").join(filename);

    write_agent(target_path, plist)
}

fn write_agent(filename: PathBuf, content: PList) -> Result<()> {
    Ok(std::fs::write(filename, content.0)?)
}

fn cmd_list_agents(matches: &ArgMatches) -> Result<()> {
    todo!()
}

fn launch_editor(file: &Path) -> Result<()> {
    todo!()
}
