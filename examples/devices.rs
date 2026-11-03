use anyhow::Result;
use igcl::device_adapter::DriverSettingScope;

fn main() -> Result<()> {
    let igcl = igcl::Igcl::new()?;

    for d in igcl.enumerate_devices()? {
        println!("{}", d.name().to_string_lossy());
        println!("\tDevice ID (LUID): {:x?}", d.device_id());
        println!("\tPCI BDF: {:?}", d.bus_device_function());
        println!("\tPCI vendor: {:#x}", d.pci_vendor_id());
        println!("\tPCI device: {:#x}", d.pci_device_id());
        println!("\tPCI subsys: {:#x}", d.pci_subsys_id());
        println!("\tPCI subsys vendor: {:#x}", d.pci_subsys_vendor_id());
        println!("\tDevice type: {:?}", d.device_type());

        println!(
            "\tEndurance gaming: {:?}",
            d.feature_endurance_gaming(DriverSettingScope::CurrentProcess)
        );
        println!(
            "\tFrame limit: {:?}",
            d.feature_frame_limit(DriverSettingScope::CurrentProcess)
        );
        println!(
            "\tFlip mode: {:?}",
            d.feature_flip_mode(DriverSettingScope::CurrentProcess)
        );

        for o in d.enumerate_display_outputs()? {
            let p = o.properties()?;
            println!("\tDisplay output:");
            #[cfg(windows)]
            println!("\t\tTarget ID: {}", p.windows_display_encoder_id());
            println!("\t\tActive: {}", p.is_active());
            println!("\t\tAttached: {}", p.is_attached());
            println!("\t\tRefresh rate: {} Hz", p.refresh_rate());
            println!(
                "\t\tAdaptive Sync supported: {}",
                p.is_adaptive_sync_supported()
            );
            println!(
                "\t\tAdaptive Sync enabled: {}",
                p.is_adaptive_sync_enabled()
            );
            println!(
                "\t\tIntel Arc Sync monitor: {:?}",
                o.intel_arc_sync_monitor_params()
            );
            println!(
                "\t\tIntel Arc Sync profile: {:?}",
                o.intel_arc_sync_profile()
            );
        }
    }

    Ok(())
}
