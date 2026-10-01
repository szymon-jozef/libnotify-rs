use libnotify_rs::api::functions::{self, uninit};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    functions::init("Morbius")?;

    if let Some(caps) = functions::get_server_caps() {
        println!("=== Server caps ===");
        for s in caps {
            println!("{}", s);
        }
    } else {
        eprintln!("No server caps");
    }

    if let Some(info) = functions::get_server_info() {
        println!("=== Server info ===");
        if let Some(name) = info.name {
            println!("Server name: {}", name);
        }

        if let Some(vendor) = info.vendor {
            println!("Server vendor: {}", vendor);
        }

        if let Some(version) = info.version {
            println!("Server version: {}", version);
        }

        if let Some(spec_version) = info.spec_version {
            println!("Server spec version: {}", spec_version);
        }
    } else {
        eprintln!("No server info");
    }

    uninit();

    Ok(())
}
