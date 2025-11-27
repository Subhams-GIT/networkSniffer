// list out all interfaces in a option format for the user to select from
use inquire::{Select, Text, validator::Validation};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use std::fmt;
use std::time::Duration;
struct StringVector(Vec<String>);
#[derive(Debug, PartialEq, Clone)]
enum Filter {
    ipAddress,
    Protocol,
    Ports,
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = match self {
            Filter::Ports => "Ports",
            Filter::Protocol => "Protocol",
            Filter::ipAddress => "ipAddress",
        };
        write!(f, "{}", label)
    }
}
impl fmt::Display for StringVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.join(", "))
    }
}

pub fn get_config() -> (Duration, String) {
    let interfaces = NetworkInterface::show().unwrap();
    let options: StringVector =
        StringVector(interfaces.iter().map(|iface| iface.name.clone()).collect());

    let interface = Select::new("select the interface using arrow buttons", options.0)
        .prompt()
        .unwrap();

    let validator = |input: &str| {
        if input.chars().count() > 1 {
            Ok(Validation::Invalid(
                "You're only allowed 1 character.".into(),
            ))
        } else {
            Ok(Validation::Valid)
        }
    };

    let timein_string = Text::new("Enter time for packets to sniff (in minutes):")
        .with_validator(validator)
        .prompt()
        .unwrap();

    let actual_time = timein_string.parse::<u64>().unwrap();
    let duration = Duration::from_secs(actual_time * 60);
    return (duration, interface);
}