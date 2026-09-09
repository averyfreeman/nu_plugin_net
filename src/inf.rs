use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{LabeledError, Record, Signature, Span, Type, Value};
use pnet::{datalink::NetworkInterface, ipnetwork::IpNetwork};

/// A Nushell command for listing network interfaces and their IP addresses.
///
/// The command is exposed as `net` and accepts no input. It returns one record
/// per interface with these fields:
///
/// - `name`: the operating-system interface name.
/// - `description`: the platform-provided interface description.
/// - `if_index`: the operating-system interface index as a Nushell integer.
/// - `mac`: the MAC address as text, or `nothing` when no MAC is available.
/// - `ips`: a table of address, address-family, and prefix-length records.
/// - `flags`: a record containing common interface state flags.
pub struct InterfacesCommand;

/// Convert a `pnet` network into the record exposed in the `ips` field.
///
/// Addresses retain their textual representation while the address family is
/// normalized to the short `v4` or `v6` labels used by the Nushell output.
fn map_ip(network: IpNetwork, span: Span) -> Value {
    let mut out = Record::with_capacity(3);

    out.push("addr", Value::string(network.ip().to_string(), span));
    out.push(
        "type",
        Value::string(
            match network {
                IpNetwork::V4(_) => "v4",
                IpNetwork::V6(_) => "v6",
            },
            span,
        ),
    );
    out.push("prefix", Value::int(i64::from(network.prefix()), span));

    Value::record(out, span)
}

/// Convert the common `pnet` interface flags into a Nushell record.
fn map_flags(inf: &NetworkInterface, span: Span) -> Value {
    let mut out = Record::with_capacity(5);

    out.push("is_up", Value::bool(inf.is_up(), span));
    out.push("is_broadcast", Value::bool(inf.is_broadcast(), span));
    out.push("is_loopback", Value::bool(inf.is_loopback(), span));
    out.push(
        "is_point_to_point",
        Value::bool(inf.is_point_to_point(), span),
    );
    out.push("is_multicast", Value::bool(inf.is_multicast(), span));

    Value::record(out, span)
}

/// Convert one `pnet` network interface into the record returned by `net`.
fn map_interface(inf: NetworkInterface, span: Span) -> Value {
    let mut o = Record::with_capacity(6);

    // Measure flags first so that we can partially move out of inf
    let flags = map_flags(&inf, span);
    let if_index = i64::from(inf.index);

    o.push("name", Value::string(inf.name, span));
    o.push("description", Value::string(inf.description, span));
    o.push("if_index", Value::int(if_index, span));
    o.push(
        "mac",
        match inf.mac {
            Some(mac) => Value::string(mac.to_string(), span),
            None => Value::nothing(span),
        },
    );
    o.push(
        "ips",
        Value::list(
            inf.ips.into_iter().map(|ip| map_ip(ip, span)).collect(),
            span,
        ),
    );
    o.push("flags", flags);

    Value::record(o, span)
}

impl SimplePluginCommand for InterfacesCommand {
    type Plugin = crate::Plugin;

    fn name(&self) -> &str {
        "net"
    }

    fn description(&self) -> &str {
        "Enumerate network interfaces on the current host"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name()).input_output_type(
            Type::Nothing,
            Type::Table(
                [
                    ("name", Type::String),
                    ("description", Type::String),
                    ("if_index", Type::Int),
                    ("mac", Type::one_of([Type::String, Type::Nothing])),
                    (
                        "ips",
                        Type::Table(
                            [
                                ("addr", Type::String),
                                ("type", Type::String),
                                ("prefix", Type::Int),
                            ]
                            .into(),
                        ),
                    ),
                    (
                        "flags",
                        Type::Record(
                            [
                                ("is_up", Type::Bool),
                                ("is_broadcast", Type::Bool),
                                ("is_loopback", Type::Bool),
                                ("is_point_to_point", Type::Bool),
                                ("is_multicast", Type::Bool),
                            ]
                            .into(),
                        ),
                    ),
                ]
                .into(),
            ),
        )
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let span = call.head;
        let interfaces = pnet::datalink::interfaces()
            .into_iter()
            .map(|interface| map_interface(interface, span))
            .collect();

        Ok(Value::list(interfaces, span))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pnet::util::MacAddr;

    fn test_span() -> Span {
        Span::test_data()
    }

    fn value_field<'a>(value: &'a Value, field: &str) -> &'a Value {
        value
            .as_record()
            .expect("expected a record")
            .get(field)
            .unwrap_or_else(|| panic!("missing field {field}"))
    }

    #[test]
    fn maps_ipv4_and_ipv6_networks() {
        let span = test_span();
        let ipv4 = map_ip("192.0.2.1/24".parse().expect("valid IPv4 network"), span);
        let ipv6 = map_ip("2001:db8::1/64".parse().expect("valid IPv6 network"), span);

        assert_eq!(value_field(&ipv4, "addr").as_str().unwrap(), "192.0.2.1");
        assert_eq!(value_field(&ipv4, "type").as_str().unwrap(), "v4");
        assert_eq!(value_field(&ipv4, "prefix").as_int().unwrap(), 24);
        assert_eq!(value_field(&ipv6, "addr").as_str().unwrap(), "2001:db8::1");
        assert_eq!(value_field(&ipv6, "type").as_str().unwrap(), "v6");
        assert_eq!(value_field(&ipv6, "prefix").as_int().unwrap(), 64);
    }

    #[test]
    fn maps_interface_fields_and_optional_mac() {
        let span = test_span();
        let interface = NetworkInterface {
            name: "example0".to_string(),
            description: "example interface".to_string(),
            index: 7,
            mac: Some(MacAddr::new(0x02, 0x00, 0x5e, 0x10, 0x00, 0x01)),
            ips: vec!["198.51.100.2/25".parse().expect("valid network")],
            flags: !0,
        };

        let mapped = map_interface(interface, span);
        assert_eq!(value_field(&mapped, "name").as_str().unwrap(), "example0");
        assert_eq!(
            value_field(&mapped, "description").as_str().unwrap(),
            "example interface"
        );
        assert_eq!(value_field(&mapped, "if_index").as_int().unwrap(), 7);
        assert_eq!(
            value_field(&mapped, "mac").as_str().unwrap(),
            "02:00:5e:10:00:01"
        );
        assert_eq!(value_field(&mapped, "ips").as_list().unwrap().len(), 1);
        let flags = value_field(&mapped, "flags");
        for field in [
            "is_up",
            "is_broadcast",
            "is_loopback",
            "is_point_to_point",
            "is_multicast",
        ] {
            assert!(
                value_field(flags, field).as_bool().unwrap(),
                "expected {field} to be true"
            );
        }

        let without_mac = NetworkInterface {
            name: "example1".to_string(),
            description: String::new(),
            index: 8,
            mac: None,
            ips: Vec::new(),
            flags: 0,
        };
        let mapped_without_mac = map_interface(without_mac, span);
        assert!(value_field(&mapped_without_mac, "mac").is_nothing());
    }

    #[test]
    fn signature_declares_optional_mac_and_nested_columns() {
        let signature = InterfacesCommand.signature();
        let output_type = signature
            .get_output_type(Some(&Type::Nothing))
            .expect("signature should declare an output type");
        let Type::Table(columns) = output_type else {
            panic!("expected table output type");
        };

        let mac_type = columns
            .iter()
            .find(|(name, _)| name == "mac")
            .map(|(_, ty)| ty)
            .expect("signature should declare mac");
        assert_eq!(
            mac_type,
            &Type::one_of([Type::String, Type::Nothing]),
            "mac must allow missing values"
        );

        let ips_type = columns
            .iter()
            .find(|(name, _)| name == "ips")
            .map(|(_, ty)| ty)
            .expect("signature should declare ips");
        let Type::Table(ip_columns) = ips_type else {
            panic!("ips should be a table");
        };
        assert_eq!(
            ip_columns
                .iter()
                .map(|(name, _)| name.as_str())
                .collect::<Vec<_>>(),
            vec!["addr", "type", "prefix"]
        );
    }
}
