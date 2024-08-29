use crate::ffi::{
    ctl_adapter_bdf_t, ctl_device_adapter_handle_t, ctl_device_adapter_properties_t,
    ctl_device_type_t,
};

#[doc(alias = "ctl_device_adapter_handle_t")]
pub struct DeviceAdapter {
    pub(crate) device_adapter_handle: ctl_device_adapter_handle_t,
    /// Note: the pointer to the device ID is invalid at this point and should not be used.
    /// Use [`Self::device_id`] instead.
    pub(crate) adapter_properties: ctl_device_adapter_properties_t,
    pub(crate) device_id: Vec<u8>,
}

impl DeviceAdapter {
    pub fn device_adapter_handle(&self) -> ctl_device_adapter_handle_t {
        self.device_adapter_handle
    }

    /// Retrieve the unique device identifier, determined by the operating system.
    /// On windows, this will be the LUID.
    pub fn device_id(&self) -> &[u8] {
        &self.device_id
    }

    /// Retrieve the PCI Bus ID, the PCI Device ID, and the PCI Device Function in that order.
    pub fn bus_device_function(&self) -> (u8, u8, u8) {
        let ctl_adapter_bdf_t {
            bus,
            device,
            function,
        } = self.adapter_properties.adapter_bdf;

        (bus, device, function)
    }

    pub fn name(&self) -> &[i8] {
        &self.adapter_properties.name
    }

    pub fn pci_vendor_id(&self) -> u32 {
        self.adapter_properties.pci_vendor_id
    }

    pub fn pci_device_id(&self) -> u32 {
        self.adapter_properties.pci_device_id
    }

    pub fn pci_subsys_vendor_id(&self) -> u16 {
        self.adapter_properties.pci_subsys_vendor_id
    }

    pub fn pci_subsys_id(&self) -> u16 {
        self.adapter_properties.pci_subsys_id
    }

    pub fn device_type(&self) -> ctl_device_type_t {
        self.adapter_properties.device_type
    }
}
