mod test;

use regex::Regex;
use std::cmp::Ordering;

use clap::Parser;
// use cli_clipboard;

#[derive(Parser)]
#[command(
    arg_required_else_help(true),
    author,
    version,
    about,
    long_about = "Format MAC addresses for easy copy-pasting. As a convience,
this program also copies the last formatted MAC addresses to your clipboard for easy pasting."
)]
struct Cli {
    /// Pass in any number of MAC addresses that you would like to format. If you're using spaces
    /// as a separator, wrap them in quotes.
    mac: Option<Vec<String>>,

    /// Format the MAC addresses in uppercase; defaults to lowercase.
    #[arg(short, long, default_value_t = false)]
    caps: bool,

    /// Use a custom separator character between each octet
    #[arg(short, long, default_value_t = String::from(":"))]
    separator: String,
}

fn filter_non_hex_chars(mac: &str) -> String {
    let sep = Regex::new(r"[[:^xdigit:]]*").unwrap();
    sep.replace_all(mac, "").to_string()
}

fn convert_case(mac: &str, caps: bool) -> String {
    if caps {
        return mac.to_uppercase();
    }
    mac.to_lowercase()
}

fn format_mac(mac: &str, caps: bool, separator: &str) -> String {
    let mac = convert_case(mac, caps);
    let re = Regex::new(r"[[:xdigit:]]{2}").unwrap();

    let mac_array: Vec<_> = re.find_iter(&mac).map(|m| m.as_str()).collect();

    mac_array.join(separator).to_string()
}

fn process_mac(mac: &str, caps: bool, separator: &str) -> Result<String, String> {
    let mac = filter_non_hex_chars(mac);
    let mac_length: u32 = mac
        .chars()
        .count()
        .try_into()
        .map_err(|_| "Conversion error")?;

    match mac_length.cmp(&12) {
        Ordering::Equal => Ok(format_mac(&mac, caps, separator)),
        Ordering::Less => Err(format!(
            "Invalid MAC address: {} is missing {} character(s)",
            &mac,
            (12 - mac.chars().count())
        )),
        Ordering::Greater => Err(format!(
            "Invalid MAC address: {} has {} extra character(s)",
            &mac,
            (mac.chars().count() - 12)
        )),
    }
}

fn main() {
    let cli = Cli::parse();

    if let Some(mac_addresses) = cli.mac {
        for mac in mac_addresses {
            match process_mac(&mac, cli.caps, &cli.separator) {
                Ok(formatted_mac) => {
                    println!("{}", formatted_mac);
                    if let Err(err) = cli_clipboard::set_contents(formatted_mac) {
                        eprintln!("Error setting clipboard contents: {}", err);
                    }
                }
                Err(err) => eprintln!("Error: {}", err),
            }
        }
    }
}
