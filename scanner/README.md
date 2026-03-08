README.md
Markdown
# Network Recon & Security Auditor (Rust)

A high-performance, asynchronous network scanner designed for internal security audits. This tool provides stealth capabilities, OS fingerprinting, and service detection while prioritizing network safety through exclusion lists and concurrency limits.

## 🛠 Prerequisites

* **Rust Toolchain:** [Install Rust](https://rustup.rs/) (v1.70+)
* **Cargo Dependencies:** Ensure `Cargo.toml` includes `tokio`, `serde`, `serde_json`, `ipnet`, `rand`, `socket2`, and `indicatif`.

## 🚀 Installation & Compilation

1. Clone the repository to your local machine.
2. Compile the production binary for maximum performance:
   ```bash
   cargo build --release
The executable will be located in ./target/release/.

📁 Required Configuration Files
For the scanner to function effectively, especially in automated or large-scale environments, it utilizes the following plain-text files:

1. blacklist.txt (Safety First)
This file is mandatory for preventing accidental scans of critical infrastructure. Any IP or range listed here will be stripped from the target list before the scan begins.

Format: One IP or CIDR per line.

Example:

Plaintext
192.168.1.1       # Primary Gateway
192.168.1.50/32    # Production Database
10.0.0.0/8         # Entire Management VLAN (Excluded)
2. IP Address Input (targets.txt)
When prompted for an IP range, you can provide a path to a .txt file instead of typing a CIDR.

Format: Supports individual IPs, comma-separated values, or CIDR notation.

Example:

Plaintext
192.168.1.0/24
172.16.0.5
172.16.0.20
3. Port Input (ports.txt)
For scanning non-standard ports or specific service lists, use a port file.

Format: One port per line or comma-separated.

Example:

Plaintext
21, 22, 25, 80, 443
8080
8443
⚙️ Key Features
🛡 Stealth Mode & Jitter
When enabled, the scanner introduces "Jitter"—a random millisecond delay between connection attempts. This breaks the rhythmic pattern that triggers Intrusion Detection Systems (IDS) and prevents the scanner from appearing as a bot.

🐧 OS Fingerprinting (Passive)
The tool analyzes the Time to Live (TTL) value in the header of the returning TCP packets.

TTL 64: Likely Linux/Unix/macOS.

TTL 128: Likely Windows.

TTL 255: Likely Cisco/Network Infrastructure.

🔍 Service Probing
For web-standard ports (80, 443, 8080), the tool automatically sends a HEAD request to capture the server header (e.g., Server: nginx/1.18.0), helping identify outdated software versions.

⚠️ Security Warning & Ethics
This tool is intended for authorized security testing only. Scanning networks you do not own or have explicit permission to test may be illegal and can cause service disruptions. Always coordinate with network administrators before running high-concurrency scans.

📊 Output
Terminal: Real-time logging of open ports and identified services.

JSON: A full report (scan_results.json) is generated upon completion for integration with other reporting tools or SIEMs.


---

### Pro-Tip for Remote Locations
If your team is using a very low-power device (like a Raspberry Pi or an old laptop), remind them to keep the **concurrency limit below 25** and keep the **Progress Bar OFF**. This minimizes CPU interrupts and context switching, ensuring the scan finishes faster despite the limited hardware.

**Is there anything else you'd like to add to the tool, such as an automatic "Aler