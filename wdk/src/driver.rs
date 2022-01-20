use alloc::boxed::Box;
use core::mem::size_of;
use core::ptr::null_mut;
use fallible_collections::FallibleBox;

use wdk_sys::base::{DRIVER_OBJECT, UNICODE_STRING};
use wdk_sys::ntoskrnl::IoCreateDevice;

use crate::device::{
    Access, Device, DeviceDoFlags, DeviceExtension, DeviceFlags, DeviceOperations,
    DeviceOperationsVtable, DeviceType,
};
use crate::error::{Error, IntoResult};

pub struct Driver {
    pub raw: *mut DRIVER_OBJECT,
}

impl Driver {
    pub unsafe fn from_raw(raw: *mut DRIVER_OBJECT) -> Self {
        Self { raw }
    }

    pub unsafe fn as_raw(&self) -> *const DRIVER_OBJECT {
        self.raw as _
    }

    pub unsafe fn as_raw_mut(&mut self) -> *mut DRIVER_OBJECT {
        self.raw as _
    }

    pub fn create_device<T>(
        &mut self,
        name: &mut UNICODE_STRING,
        device_type: DeviceType,
        device_flags: DeviceFlags,
        device_do_flags: DeviceDoFlags,
        access: Access,
        data: T,
    ) -> Result<Device, Error>
    where
        T: DeviceOperations,
    {
        // Box the data.
        let data = <Box<_> as FallibleBox<_>>::try_new(data)?;

        // Create the device.
        let mut device = null_mut();

        unsafe {
            IoCreateDevice(
                self.raw,
                size_of::<DeviceExtension>() as u32,
                name,
                device_type.into(),
                device_flags.bits(),
                access.is_exclusive() as _,
                &mut device,
            )
        }
        .into_result()?;

        unsafe {
            (*device).Flags |= device_do_flags.bits();
        }

        let device = unsafe { Device::from_raw(device) };

        // Store the boxed data and vtable.
        let extension = device.extension_mut();
        extension.device_type = device_type;
        extension.vtable = &DeviceOperationsVtable::<T>::VTABLE;
        extension.data = Box::into_raw(data) as *mut cty::c_void;

        Ok(device)
    }
}
