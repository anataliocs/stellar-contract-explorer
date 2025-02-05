use std::time::Duration;

use xshell::{cmd, Shell};

use crate::commands::commands::{StellarCliCmd, StellarCliCmdName};

pub mod commands {
    use std::time::Duration;
    use xshell::{cmd, Shell};
    use StellarCliCmdName::{Env, NetworkToggle};
    use crate::commands::commands::StellarCliCmdName::{ReadContractDataWasm, Version};

    pub enum StellarCliCmdName {
        Version,
        Env,
        ReadContractDataWasm,
        NetworkToggle
    }

    impl StellarCliCmdName {
        pub fn get_cmd(stellar_cli_cmd_name: &StellarCliCmdName) -> StellarCliCmd {
            match stellar_cli_cmd_name {
                Version => {
                    let version = "--version";
                    StellarCliCmd::new(Version, cmd!(get_shell(), "stellar {version}"))
                },
                Env => {
                    let options = "--global";
                    StellarCliCmd::new(Version, cmd!(get_shell(), "stellar env {options}"))
                },
                ReadContractDataWasm => {
                    let options = "--output json --id CBQDHNBFBZYE4MKPWBSJOPIYLW4SFSXAXUTSXJN76GNKYVYPCKWC6QUK --wasm 26c495019afb7448f690a82d6e66d8fab1ad3fd3e7b4aec7d554209966c9d19d --durability persistent";
                    StellarCliCmd::new(Version, cmd!(get_shell(), "stellar contract read {options}"))
                }
                NetworkToggle => {
                    StellarCliCmd::new(Version, cmd!(get_shell(), "stellar network use local"))
                }

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

    pub fn execute(stellar_cli_cmd: StellarCliCmdName) -> String {
        let cmd =  command_factory(&stellar_cli_cmd);


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
        StellarCliCmdName::get_cmd(stellar_cli_cmd)
    }
}

