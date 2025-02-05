use std::time::Duration;

use xshell::Shell;

use crate::commands::commands::{StellarCliCmd, StellarCliCmdName};

pub mod commands {
    use xshell::cmd;

    use crate::commands::commands::StellarCliCmdName::VERSION;
    use crate::commands::get_shell;

    pub enum StellarCliCmdName {
        VERSION,
    }

    impl StellarCliCmdName {
        pub fn get_cmd(&self) -> StellarCliCmd {
            match &self {
                VERSION => StellarCliCmd::new(VERSION, cmd!(get_shell(), "stellar --version")),
            }
        }
    }

    pub struct StellarCliCmd {
        pub stellar_cli_cmd_name: StellarCliCmdName,
        pub cmd_slug: xshell::Cmd,
    }

    impl StellarCliCmd {
        pub fn new(stellar_cli_cmd_name: StellarCliCmdName, cmd_slug: xshell::Cmd) -> Self {
            Self {
                stellar_cli_cmd_name,
                cmd_slug,
            }
        }
    }
}
pub fn execute(stellar_cli_cmd: StellarCliCmdName) -> String {
    let cmd = match &stellar_cli_cmd {
        StellarCliCmdName::VERSION => command_factory(&stellar_cli_cmd),
    };

    // Run the command with a timeout
    cmd.cmd_slug
       .timeout(Duration::from_secs(3))
       .read()
       .unwrap_or_else(|_e| {
           "".parse().unwrap()
       })
}

fn get_shell() -> Shell {
    Shell::new().unwrap()
}

fn command_factory(stellar_cli_cmd: &StellarCliCmdName) -> StellarCliCmd {
    let cmd_string = stellar_cli_cmd.get_cmd();
    let cmd = cmd_string;

    // Setup env
    //cmd.env("key", "value");

    // Setup args
    //cmd.args()

    cmd
}
