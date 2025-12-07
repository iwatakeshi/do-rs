//! Example demonstrating how to create a Droplet with SSH keys
//!
//! This example shows how to use the SSH keys union type that supports
//! both integer IDs and string fingerprints.

use digitalocean::models::{DropletCreate, DropletCreateImage, DropletCreateSshKeysInner};

fn main() {
    // Create a new droplet configuration
    let image = DropletCreateImage::String("ubuntu-22-04-x64".to_string());
    let mut droplet = DropletCreate::new("s-1vcpu-1gb".to_string(), image);

    // Set the region
    droplet.region = Some("nyc3".to_string());

    // Add SSH keys using different formats
    droplet.ssh_keys = Some(vec![
        // Using integer ID (e.g., from your DigitalOcean account)
        DropletCreateSshKeysInner::from_id(123456),
        
        // Using fingerprint string
        DropletCreateSshKeysInner::from_fingerprint(
            "aa:bb:cc:dd:ee:ff:00:11:22:33:44:55:66:77:88:99".to_string()
        ),
        
        // Using the From trait for convenience (integer)
        789012i64.into(),
        
        // Using the From trait for convenience (string)
        "11:22:33:44:55:66:77:88:99:aa:bb:cc:dd:ee:ff:00".into(),
    ]);

    // Serialize to JSON to see the output format
    let json = serde_json::to_string_pretty(&droplet).unwrap();
    println!("Droplet creation request JSON:");
    println!("{}", json);

    // The ssh_keys field will be serialized as:
    // "ssh_keys": [
    //   123456,
    //   "aa:bb:cc:dd:ee:ff:00:11:22:33:44:55:66:77:88:99",
    //   789012,
    //   "11:22:33:44:55:66:77:88:99:aa:bb:cc:dd:ee:ff:00"
    // ]
}
