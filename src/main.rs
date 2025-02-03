use std::any::Any;
use std::char::REPLACEMENT_CHARACTER;
use std::os::unix::process::CommandExt;

use promkit::crossterm::style::{Attribute, Attributes, Color, Stylize};
use promkit::grapheme::StyledGraphemes;
use promkit::preset::confirm::Confirm;
use promkit::preset::json::Json;
use promkit::preset::listbox::Listbox;
use promkit::preset::readline::Readline;
use promkit::style::StyleBuilder;

use crate::json_util::json_string;

mod form_one;
mod json_util;
mod key_map;

fn main() -> anyhow::Result<()> {
    let base_prompt =
        Confirm::new("This command line tool will walk through executing various scripts to perform Extend TTL and Restore Archived data commands")
            .prompt()?.run();

    let listbox = Listbox::new(Vec::from([
        StyledGraphemes::from("Extend TTL"),
        StyledGraphemes::from("Restore Archived Data"),
    ]))
    .title("Choose operation to perform")
    .title_style(
        StyleBuilder::new()
            .attrs(Attributes::from(Attribute::Underlined).with(Attribute::Bold))
            .build()
            .stylize(),
    )
    .listbox_lines(3)
    .prompt()
    .unwrap()
    .run();

    let listbox_two = Listbox::new(Vec::from([
        StyledGraphemes::from("Extend instance TTL"),
        StyledGraphemes::from("Extend persistence TTL"),
    ]))
    .title("Extend TTL for what time of Smart Contract Storage?")
    .title_style(
        StyleBuilder::new()
            .attrs(Attributes::from(Attribute::Underlined).with(Attribute::Bold))
            .build()
            .stylize(),
    )
    .listbox_lines(3)
    .prompt()
    .unwrap()
    .run();

    Readline::default()
        .title("Enter Deployed CONTRACT_ID")
        .prefix("Contract ID ❯❯ ")
        .prefix_style(
            StyleBuilder::new()
                .attrs(Attributes::from(Attribute::Italic).with(Attribute::Bold))
                .fgc(Color::Green)
                .build()
                .stylize(),
        )
        .prompt()
        .unwrap()
        .run()?;
    Readline::default()
        .title("Enter Secret Key for testnet")
        .prefix("Secret Key ❯❯ ")
        .mask(REPLACEMENT_CHARACTER)
        .prompt()
        .unwrap()
        .run()?;
    Readline::default()
        .title("Enter Public Key for testnet")
        .prefix("Public Key ❯❯ ")
        .prompt()
        .unwrap()
        .run()?;

    println!("\n❯❯ Initiating command...");
    println!(
        "Executing ❯❯ pnpx ts-node extendPersistentTtl.ts [CONTRACT_ID] [SOURCE_KEYPAIR] [PERSISTENT_STORAGE_KEY]"
    );

    Json::new(json_string())
        .title("Evaluate JSON Response (ctrl+c to exit)")
        .active_item_attribute(Attribute::Bold)
        .inactive_item_attribute(Attribute::Dim)
        .indent(4)
        .json_lines(60)
        .prompt()
        .unwrap()
        .run()?;

    Ok(())
}
