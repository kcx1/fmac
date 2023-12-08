use clap::Parser;
use cli_clipboard;
use regex::Regex;
use std::cmp::Ordering;

#[derive(Parser)]
#[command(
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
    #[arg(short, long, default_value_t = ':')]
    separator: char,
}

fn filter_seperators(mac: &str) -> String {
    let sep = Regex::new(r"[[:^xdigit:]]*").unwrap();
    sep.replace_all(mac, "").to_string()
}

fn capitalize(mac: &str, caps: bool) -> String {
    if caps {
        return mac.to_uppercase();
    }
    mac.to_lowercase()
}

fn format_mac(mac: &str, caps: bool, separator: char) -> String {
    let mac = capitalize(mac, caps);
    let re = Regex::new(r"[[:xdigit:]]{2}").unwrap();

    let mac_array: Vec<_> = re.find_iter(&mac).map(|m| m.as_str()).collect();

    mac_array.join(&separator.to_string()).to_string()
}

fn process_mac(mac: &str, caps: bool, separator: char) -> String {
    let mac = filter_seperators(&mac);
    let mac_length: u32 = mac.chars().count().try_into().unwrap();
    match mac_length.cmp(&12) {
        Ordering::Equal => return format_mac(&mac, caps, separator),
        Ordering::Less => {
            return format!(
                "Invalid MAC address: {} is missing {} character(s)",
                &mac,
                (12 - mac.chars().count())
            )
        }
        Ordering::Greater => {
            return format!(
                "Invalid MAC address: {} has {} extra character(s)",
                &mac,
                (mac.chars().count() - 12)
            )
        }
    }
}

fn main() {
    let cli = Cli::parse();
    for mac in cli.mac.unwrap() {
        let mac = process_mac(&mac, cli.caps, cli.separator);
        println!("{}", mac);
        cli_clipboard::set_contents(mac).unwrap();
    }
}
