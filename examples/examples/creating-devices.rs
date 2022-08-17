#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

use wdk::device::{
    dispatch_device, Access, Completion, Device, DeviceDoFlags, DeviceFlags, DeviceOperations,
    DeviceType, RequestError,
};
use wdk::driver::Driver;
use wdk::error::Error;
use wdk::ioctl::RequiredAccess;
use wdk::request::{IoControlRequest, IoRequest, ReadRequest, WriteRequest};
use wdk::{println, unicode_string};
use wdk_sys::base::{
    DRIVER_OBJECT, IRP_MJ_MAXIMUM_FUNCTION, NTSTATUS, STATUS_SUCCESS, UNICODE_STRING,
};

const IOCTL_PRINT_VALUE: u32 = 0x800;
const IOCTL_READ_VALUE: u32 = 0x801;
const IOCTL_WRITE_VALUE: u32 = 0x802;

struct MyDevice {
    data: Vec<u8>,
    value: u32,
}

impl MyDevice {
    fn print_value(&mut self, _request: &IoControlRequest) -> Result<u32, Error> {
        println!("value: {}", self.value);

        Ok(0)
    }

    fn read_value(&mut self, request: &IoControlRequest) -> Result<u32, Error> {
        let mut user_ptr = request.user_ptr();

        user_ptr.write(&self.value)?;

        Ok(core::mem::size_of::<u32>() as u32)
    }

    fn write_value(&mut self, request: &IoControlRequest) -> Result<u32, Error> {
        let user_ptr = request.user_ptr();

        self.value = user_ptr.read()?;

        Ok(0)
    }
}

impl DeviceOperations for MyDevice {
    fn create(&mut self, _device: &Device, request: IoRequest) -> Result<Completion, RequestError> {
        println!("userspace opened the device");

        Ok(Completion::Complete(0, request))
    }

    fn close(&mut self, _device: &Device, request: IoRequest) -> Result<Completion, RequestError> {
        println!("userspace closed the device");

        Ok(Completion::Complete(0, request))
    }

    fn cleanup(
        &mut self,
        _device: &Device,
        request: IoRequest,
    ) -> Result<Completion, RequestError> {
        println!("device is no longer in use by userspace");

        Ok(Completion::Complete(0, request))
    }

    fn read(&mut self, _device: &Device, request: ReadRequest) -> Result<Completion, RequestError> {
        let mut user_ptr = request.user_ptr();
        let slice = user_ptr.as_mut_slice();

        let offset = (request.offset() as usize).min(self.data.len());
        let size = slice.len().min(self.data.len() - offset);

        slice[0..size].copy_from_slice(&self.data[offset..offset + size]);

        Ok(Completion::Complete(size as u32, request.into()))
    }

    fn write(
        &mut self,
        _device: &Device,
        request: WriteRequest,
    ) -> Result<Completion, RequestError> {
        let user_ptr = request.user_ptr();

        if request.offset() > 0 {
            return Err(RequestError(Error::END_OF_FILE, request.into()))?;
        }

        let slice = user_ptr.as_slice();
        let size = slice.len().min(4096);

        self.data = slice[0..size].to_vec();

        Ok(Completion::Complete(size as u32, request.into()))
    }

    fn ioctl(
        &mut self,
        _device: &Device,
        request: IoControlRequest,
    ) -> Result<Completion, RequestError> {
        let result = match request.function() {
            (_, IOCTL_PRINT_VALUE) => self.print_value(&request),
            (RequiredAccess::READ_DATA, IOCTL_READ_VALUE) => self.read_value(&request),
            (RequiredAccess::WRITE_DATA, IOCTL_WRITE_VALUE) => self.write_value(&request),
            _ => Err(Error::INVALID_PARAMETER),
        };

        match result {
            Ok(size) => Ok(Completion::Complete(size, request.into())),
            Err(e) => Err(RequestError(e, request.into())),
        }
    }
}

#[no_mangle]
extern "system" fn driver_entry(driver: &mut DRIVER_OBJECT, _: &UNICODE_STRING) -> NTSTATUS {
    driver.DriverUnload = Some(driver_exit);

    let mut drv = unsafe { Driver::from_raw(driver) };

    drv.create_device(
        &mut unicode_string!("\\Device\\Example"),
        DeviceType::Unknown,
        DeviceFlags::SECURE_OPEN,
        DeviceDoFlags::DO_BUFFERED_IO,
        Access::NonExclusive,
        MyDevice {
            data: vec![],
            value: 0,
        },
    )
    .unwrap();

    for i in 0..IRP_MJ_MAXIMUM_FUNCTION {
        driver.MajorFunction[i as usize] = Some(dispatch_device);
    }

    STATUS_SUCCESS
}

extern "stdcall" fn driver_exit(_driver: *mut DRIVER_OBJECT) {}
