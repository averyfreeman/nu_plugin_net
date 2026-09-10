# Feature assessment: packet capture and ARP discovery

**Status:** Tabled discovery; no feature implementation is proposed in this
change.

## Executive conclusion

A focused first version is more straightforward to implement directly in the
Nushell plugin than to build around the existing applications. The local
`arp-scan-rs` checkout is a feature-rich, binary-only command-line program,
and the useful part of `rustnet` is a capture crate inside a TUI-oriented
workspace rather than a ready-made headless plugin backend.

The recommended future direction is therefore:

1. Keep the existing `net` command and add separate, bounded commands for
   sniffing and ARP discovery.
2. Use a packet-capture backend directly, preferably libpcap/Npcap through the
   Rust `pcap` crate when cross-platform capture is required. Linux-only
   capture can use the existing pnet datalink support, whose Linux backend is
   based on AF_PACKET.
3. Export standard PCAP or PCAPNG files. Wireshark is the primary target;
   tcpdump compatibility should follow from using standard libpcap formats and
   correct link-layer metadata, without adding a tcpdump-specific mode.
4. Use pnet directly for a minimal IPv4 ARP scanner. Treat external
   `arp-scan-rs` execution as an optional later integration only if its mature
   scanning profiles and enrichment features are more valuable than a
   self-contained plugin.

Relevant references: [pnet datalink documentation](https://docs.rs/pnet/latest/pnet/datalink/index.html),
[Linux packet sockets](https://www.man7.org/linux/man-pages/man7/packet.7.html),
[Rust pcap](https://docs.rs/pcap/latest/pcap/),
[pcap-file](https://docs.rs/pcap-file/latest/pcap_file/), and the
[Wireshark User's Guide](https://www.wireshark.org/docs/wsug_html/).

## Rating scale

- **Plausibility:** 1 means unlikely with the current project; 5 means the
  capability fits the existing architecture well.
- **Friction:** 1 means low implementation and operational friction; 5 means
  substantial platform, packaging, privilege, lifecycle, or maintenance work.

These are engineering estimates for a useful first version, not estimates for
full parity with either external project.

## Feature ratings

| Proposal or approach | Plausibility | Friction | Assessment |
| --- | ---: | ---: | --- |
| Direct packet sniffing on Linux with pnet/AF_PACKET | 4/5 | 4/5 | Technically compatible with the current dependency set, but requires raw-socket privileges, bounded streaming behavior, cancellation, and careful handling of link-layer frames. |
| Direct portable packet sniffing through libpcap/Npcap | 4/5 | 4/5 | Better platform coverage and BPF support, but adds native library/toolchain requirements and another backend dependency. |
| Reusing `rustnet-capture` as a library | 4/5 | 4/5 | Its `CaptureConfig`/`PacketReader` surface is useful, but the crate is part of a workspace, depends on libpcap, and would need a deliberately pinned and maintained dependency boundary. |
| Launching the `rustnet` executable | 3/5 | 5/5 | The binary is a TUI application, not a headless capture service. Child-process lifecycle, terminal behavior, output files, and shutdown would be awkward inside a Nushell plugin. |
| Direct PCAP export | 5/5 | 2/5 | A small writer can preserve packet bytes, timestamps, link type, captured length, and original length in a widely supported format. |
| Direct PCAPNG export | 4/5 | 3/5 | Feasible and more extensible, but requires section/interface/enhanced-packet block handling and more validation than classic PCAP. |
| Direct minimal IPv4 ARP scan | 4/5 on Linux | 3/5 | pnet already provides Ethernet/ARP packet construction and datalink channels. The first version can remain small: explicit interface, CIDR/range, timeout, retries, deduplication, and structured replies. |
| Direct full `arp-scan-rs` parity | 3/5 | 5/5 | Profiles, VLANs, custom fields, bandwidth controls, DNS, OUI lookup, and multiple serializers substantially enlarge the API and test surface. |
| Invoke `arp-scan-rs` as an external application | 4/5 if installed | 4/5 | Reduces packet-level implementation work, but adds executable discovery, CLI/version drift, process timeout and cancellation, stderr handling, output parsing, installation requirements, and license/distribution review. |

## Evaluation of the local repositories

### `arp-scan-rs` `v0.15.1`

The checkout is a binary-only Cargo package declaring `AGPL-3.0-or-later`.
Its implementation in `src/main.rs` and the `network` module is a useful
reference for ARP packet construction, response collection, deduplication, and
optional DNS/vendor enrichment. It uses pnet 0.35 and includes Linux raw-
network capability checks.

It is not a clean library dependency: the application parses CLI arguments,
prints results, uses process exits for some failures, and owns its receive and
send lifecycle. Invoking it from the plugin would require locating a separately
installed executable and parsing one of its JSON/YAML/CSV modes. The plugin
would still need to manage interface and range validation, cancellation,
timeouts, stderr, exit status, and incompatible version changes. Copying or
linking implementation code would also require resolving the project's AGPL
licensing implications for this crate.

### `rustnet` `v1.6.0`

The workspace is Apache-2.0 licensed. Its `rustnet-capture` crate exposes a
more promising abstraction around libpcap/Npcap: capture configuration,
interface validation, packet reads, timestamps, and original lengths. However,
it depends on a native capture library and is maintained inside a larger
workspace whose main product is an interactive TUI.

The root application’s `--pcap-export` and `--pcapng-export` options are not a
small headless service API. The PCAPNG implementation is tied to the
application and internal export modules. Running the executable from a plugin
would inherit the TUI and process-lifecycle problems. Directly using the
`pcap` API, or extracting and stabilizing a narrowly scoped capture crate if
future reuse becomes important, is simpler than integrating the whole
application.

## Direct implementation versus existing applications

Direct implementation wins for the minimum useful feature set because this
plugin already has pnet, needs Nushell-native structured output, and can keep
the command surface deliberately small. It also avoids making a Nushell
command depend on a user-installed binary that may not exist on the target
machine.

External applications become attractive only for intentionally delegated
feature parity. `arp-scan-rs` could provide mature scan profiles and enrichment
if users are willing to install it and the project accepts an adapter with
runtime and licensing constraints. `rustnet` could be a reference for capture
statistics and PCAPNG behavior, but its executable should not be the default
backend. A future standalone `rustnet-capture` dependency would need an
explicit version/license/native-dependency policy.

## Suggested future boundaries

### `net sniff`

- Use a full Nushell streaming plugin command rather than a simple command;
  long-running packet output should be cancellable and bounded.
- Require or clearly select an interface, with optional packet count, duration,
  snap length, and BPF filter.
- Write PCAP or PCAPNG to an explicit path and report a structured summary when
  capture ends.
- Preserve packet bytes and capture metadata; do not buffer an unbounded
  capture in Nushell memory.
- Give explicit privilege and native-library errors. Avoid enabling
  promiscuous mode implicitly in the first version.

### `net arp-scan`

- Require an interface and an IPv4 CIDR or explicit address range.
- Send broadcast ARP requests, listen for replies, deduplicate by address/MAC,
  and return structured `ip` and `mac` records.
- Bound timeout, retries, and request rate; report partial results separately
  from fatal configuration or privilege errors.
- Defer DNS, OUI databases, VLAN customization, scan profiles, and additional
  serializers until the minimal behavior is stable.
- Document that scanning must be authorized on the local network.

## Verification plan for a future implementation

- Unit-test Ethernet and ARP packet construction/parsing with deterministic
  byte fixtures, IPv4 range expansion, reply deduplication, timeout behavior,
  and privilege/error mapping.
- Generate PCAP and PCAPNG fixtures, reopen them with a parser, and verify link
  type, timestamps, captured/original lengths, and packet bytes.
- Add a Linux-only integration test using an isolated network namespace when
  the runner provides the required capabilities; do not scan a real LAN in
  default CI.
- Perform a manual acceptance test with Nushell 0.115.x: capture a bounded
  file, open it in Wireshark, and optionally read it with tcpdump when
  available; then run an authorized ARP scan and verify structured rows.
- If external adapters are ever added, test them with a fake executable and
  fixed JSON fixtures for missing binaries, non-zero exits, malformed output,
  timeouts, and cancellation.

This assessment intentionally leaves implementation, documentation-site
changes, and GitHub Actions changes out of scope.
