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
            "\tfeature_flip_mode: {:?}",
            d.feature_flip_mode(DriverSettingScope::Global)
        );
        println!(
            "\tfeature_endurance_gaming: {:?}",
            d.feature_endurance_gaming(DriverSettingScope::Global)
        );
        println!(
            "\tfeature_frame_limit: {:?}",
            d.feature_frame_limit(DriverSettingScope::Global)
        );

        dbg!(d.power_telemetry());

        // d.device_adapter_handle()
    }

    Ok(())
}
