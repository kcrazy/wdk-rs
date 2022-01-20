use alloc::boxed::Box;
use bitflags::bitflags;
use core::marker::PhantomData;
use core::ptr::null_mut;

use wdk_sys::base::{DEVICE_OBJECT, IRP, NTSTATUS, STATUS_SUCCESS};
use wdk_sys::base::{
    IRP_MJ_CLEANUP, IRP_MJ_CLOSE, IRP_MJ_CREATE, IRP_MJ_DEVICE_CONTROL, IRP_MJ_READ, IRP_MJ_WRITE,
};
use wdk_sys::ntoskrnl::{IoDeleteDevice, IoGetCurrentIrpStackLocation};

use crate::error::Error;
use crate::request::{IoControlRequest, IoRequest, ReadRequest, WriteRequest};

#[derive(Copy, Clone, Debug)]
pub enum Access {
    NonExclusive,
    Exclusive,
}

impl Access {
    pub fn is_exclusive(&self) -> bool {
        match *self {
            Access::Exclusive => true,
            _ => false,
        }
    }
}

bitflags! {
    pub struct DeviceFlags: u32 {
        const SECURE_OPEN = wdk_sys::base::FILE_DEVICE_SECURE_OPEN;
    }
}

bitflags! {
    pub struct DeviceDoFlags: u32 {
        const DO_BUFFERED_IO = wdk_sys::base::DO_BUFFERED_IO;
        const DO_DIRECT_IO   = wdk_sys::base::DO_DIRECT_IO;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeviceType {
    Port8042,
    Acpi,
    Battery,
    Beep,
    BusExtender,
    Cdrom,
    CdromFileSystem,
    Changer,
    Controller,
    DataLink,
    Dfs,
    DfsFileSystem,
    DfsVolume,
    Disk,
    DiskFileSystem,
    Dvd,
    FileSystem,
    Fips,
    FullscreenVideo,
    InportPort,
    Keyboard,
    Ks,
    Ksec,
    Mailslot,
    MassStorage,
    MidiIn,
    MidiOut,
    Modem,
    Mouse,
    MultiUncProvider,
    NamedPipe,
    Network,
    NetworkBrowser,
    NetworkFileSystem,
    NetworkRedirector,
    Null,
    ParallelPort,
    PhysicalNetcard,
    Printer,
    Scanner,
    Screen,
    Serenum,
    SerialPort,
    SerialMousePort,
    Smartcard,
    Smb,
    Sound,
    Streams,
    Tape,
    TapeFileSystem,
    Termsrv,
    Transport,
    Unknown,
    Vdm,
    Video,
    VirtualDisk,
    WaveIn,
    WaveOut,
}

impl Into<u32> for DeviceType {
    fn into(self) -> u32 {
        match self {
            DeviceType::Port8042 => wdk_sys::base::FILE_DEVICE_8042_PORT,
            DeviceType::Acpi => wdk_sys::base::FILE_DEVICE_ACPI,
            DeviceType::Battery => wdk_sys::base::FILE_DEVICE_BATTERY,
            DeviceType::Beep => wdk_sys::base::FILE_DEVICE_BEEP,
            DeviceType::BusExtender => wdk_sys::base::FILE_DEVICE_BUS_EXTENDER,
            DeviceType::Cdrom => wdk_sys::base::FILE_DEVICE_CD_ROM,
            DeviceType::CdromFileSystem => wdk_sys::base::FILE_DEVICE_CD_ROM_FILE_SYSTEM,
            DeviceType::Changer => wdk_sys::base::FILE_DEVICE_CHANGER,
            DeviceType::Controller => wdk_sys::base::FILE_DEVICE_CONTROLLER,
            DeviceType::DataLink => wdk_sys::base::FILE_DEVICE_DATALINK,
            DeviceType::Dfs => wdk_sys::base::FILE_DEVICE_DFS,
            DeviceType::DfsFileSystem => wdk_sys::base::FILE_DEVICE_DFS_FILE_SYSTEM,
            DeviceType::DfsVolume => wdk_sys::base::FILE_DEVICE_DFS_VOLUME,
            DeviceType::Disk => wdk_sys::base::FILE_DEVICE_DISK,
            DeviceType::DiskFileSystem => wdk_sys::base::FILE_DEVICE_DISK_FILE_SYSTEM,
            DeviceType::Dvd => wdk_sys::base::FILE_DEVICE_DVD,
            DeviceType::FileSystem => wdk_sys::base::FILE_DEVICE_FILE_SYSTEM,
            DeviceType::Fips => wdk_sys::base::FILE_DEVICE_FIPS,
            DeviceType::FullscreenVideo => wdk_sys::base::FILE_DEVICE_FULLSCREEN_VIDEO,
            DeviceType::InportPort => wdk_sys::base::FILE_DEVICE_INPORT_PORT,
            DeviceType::Keyboard => wdk_sys::base::FILE_DEVICE_KEYBOARD,
            DeviceType::Ks => wdk_sys::base::FILE_DEVICE_KS,
            DeviceType::Ksec => wdk_sys::base::FILE_DEVICE_KSEC,
            DeviceType::Mailslot => wdk_sys::base::FILE_DEVICE_MAILSLOT,
            DeviceType::MassStorage => wdk_sys::base::FILE_DEVICE_MASS_STORAGE,
            DeviceType::MidiIn => wdk_sys::base::FILE_DEVICE_MIDI_IN,
            DeviceType::MidiOut => wdk_sys::base::FILE_DEVICE_MIDI_OUT,
            DeviceType::Modem => wdk_sys::base::FILE_DEVICE_MODEM,
            DeviceType::Mouse => wdk_sys::base::FILE_DEVICE_MOUSE,
            DeviceType::MultiUncProvider => wdk_sys::base::FILE_DEVICE_MULTI_UNC_PROVIDER,
            DeviceType::NamedPipe => wdk_sys::base::FILE_DEVICE_NAMED_PIPE,
            DeviceType::Network => wdk_sys::base::FILE_DEVICE_NETWORK,
            DeviceType::NetworkBrowser => wdk_sys::base::FILE_DEVICE_NETWORK_BROWSER,
            DeviceType::NetworkFileSystem => wdk_sys::base::FILE_DEVICE_NETWORK_FILE_SYSTEM,
            DeviceType::NetworkRedirector => wdk_sys::base::FILE_DEVICE_NETWORK_REDIRECTOR,
            DeviceType::Null => wdk_sys::base::FILE_DEVICE_NULL,
            DeviceType::ParallelPort => wdk_sys::base::FILE_DEVICE_PARALLEL_PORT,
            DeviceType::PhysicalNetcard => wdk_sys::base::FILE_DEVICE_PHYSICAL_NETCARD,
            DeviceType::Printer => wdk_sys::base::FILE_DEVICE_PRINTER,
            DeviceType::Scanner => wdk_sys::base::FILE_DEVICE_SCANNER,
            DeviceType::Screen => wdk_sys::base::FILE_DEVICE_SCREEN,
            DeviceType::Serenum => wdk_sys::base::FILE_DEVICE_SERENUM,
            DeviceType::SerialMousePort => wdk_sys::base::FILE_DEVICE_SERIAL_MOUSE_PORT,
            DeviceType::SerialPort => wdk_sys::base::FILE_DEVICE_SERIAL_PORT,
            DeviceType::Smartcard => wdk_sys::base::FILE_DEVICE_SMARTCARD,
            DeviceType::Smb => wdk_sys::base::FILE_DEVICE_SMB,
            DeviceType::Sound => wdk_sys::base::FILE_DEVICE_SOUND,
            DeviceType::Streams => wdk_sys::base::FILE_DEVICE_STREAMS,
            DeviceType::Tape => wdk_sys::base::FILE_DEVICE_TAPE,
            DeviceType::TapeFileSystem => wdk_sys::base::FILE_DEVICE_TAPE_FILE_SYSTEM,
            DeviceType::Termsrv => wdk_sys::base::FILE_DEVICE_TERMSRV,
            DeviceType::Transport => wdk_sys::base::FILE_DEVICE_TRANSPORT,
            DeviceType::Unknown => wdk_sys::base::FILE_DEVICE_UNKNOWN,
            DeviceType::Vdm => wdk_sys::base::FILE_DEVICE_VDM,
            DeviceType::Video => wdk_sys::base::FILE_DEVICE_VIDEO,
            DeviceType::VirtualDisk => wdk_sys::base::FILE_DEVICE_VIRTUAL_DISK,
            DeviceType::WaveIn => wdk_sys::base::FILE_DEVICE_WAVE_IN,
            DeviceType::WaveOut => wdk_sys::base::FILE_DEVICE_WAVE_OUT,
        }
    }
}

impl From<u32> for DeviceType {
    fn from(value: u32) -> Self {
        match value {
            wdk_sys::base::FILE_DEVICE_8042_PORT => DeviceType::Port8042,
            wdk_sys::base::FILE_DEVICE_ACPI => DeviceType::Acpi,
            wdk_sys::base::FILE_DEVICE_BATTERY => DeviceType::Battery,
            wdk_sys::base::FILE_DEVICE_BEEP => DeviceType::Beep,
            wdk_sys::base::FILE_DEVICE_BUS_EXTENDER => DeviceType::BusExtender,
            wdk_sys::base::FILE_DEVICE_CD_ROM => DeviceType::Cdrom,
            wdk_sys::base::FILE_DEVICE_CD_ROM_FILE_SYSTEM => DeviceType::CdromFileSystem,
            wdk_sys::base::FILE_DEVICE_CHANGER => DeviceType::Changer,
            wdk_sys::base::FILE_DEVICE_CONTROLLER => DeviceType::Controller,
            wdk_sys::base::FILE_DEVICE_DATALINK => DeviceType::DataLink,
            wdk_sys::base::FILE_DEVICE_DFS => DeviceType::Dfs,
            wdk_sys::base::FILE_DEVICE_DFS_FILE_SYSTEM => DeviceType::DfsFileSystem,
            wdk_sys::base::FILE_DEVICE_DFS_VOLUME => DeviceType::DfsVolume,
            wdk_sys::base::FILE_DEVICE_DISK => DeviceType::Disk,
            wdk_sys::base::FILE_DEVICE_DISK_FILE_SYSTEM => DeviceType::DiskFileSystem,
            wdk_sys::base::FILE_DEVICE_DVD => DeviceType::Dvd,
            wdk_sys::base::FILE_DEVICE_FILE_SYSTEM => DeviceType::FileSystem,
            wdk_sys::base::FILE_DEVICE_FIPS => DeviceType::Fips,
            wdk_sys::base::FILE_DEVICE_FULLSCREEN_VIDEO => DeviceType::FullscreenVideo,
            wdk_sys::base::FILE_DEVICE_INPORT_PORT => DeviceType::InportPort,
            wdk_sys::base::FILE_DEVICE_KEYBOARD => DeviceType::Keyboard,
            wdk_sys::base::FILE_DEVICE_KS => DeviceType::Ks,
            wdk_sys::base::FILE_DEVICE_KSEC => DeviceType::Ksec,
            wdk_sys::base::FILE_DEVICE_MAILSLOT => DeviceType::Mailslot,
            wdk_sys::base::FILE_DEVICE_MASS_STORAGE => DeviceType::MassStorage,
            wdk_sys::base::FILE_DEVICE_MIDI_IN => DeviceType::MidiIn,
            wdk_sys::base::FILE_DEVICE_MIDI_OUT => DeviceType::MidiOut,
            wdk_sys::base::FILE_DEVICE_MODEM => DeviceType::Modem,
            wdk_sys::base::FILE_DEVICE_MOUSE => DeviceType::Mouse,
            wdk_sys::base::FILE_DEVICE_MULTI_UNC_PROVIDER => DeviceType::MultiUncProvider,
            wdk_sys::base::FILE_DEVICE_NAMED_PIPE => DeviceType::NamedPipe,
            wdk_sys::base::FILE_DEVICE_NETWORK => DeviceType::Network,
            wdk_sys::base::FILE_DEVICE_NETWORK_BROWSER => DeviceType::NetworkBrowser,
            wdk_sys::base::FILE_DEVICE_NETWORK_FILE_SYSTEM => DeviceType::NetworkFileSystem,
            wdk_sys::base::FILE_DEVICE_NETWORK_REDIRECTOR => DeviceType::NetworkRedirector,
            wdk_sys::base::FILE_DEVICE_NULL => DeviceType::Null,
            wdk_sys::base::FILE_DEVICE_PARALLEL_PORT => DeviceType::ParallelPort,
            wdk_sys::base::FILE_DEVICE_PHYSICAL_NETCARD => DeviceType::PhysicalNetcard,
            wdk_sys::base::FILE_DEVICE_PRINTER => DeviceType::Printer,
            wdk_sys::base::FILE_DEVICE_SCANNER => DeviceType::Scanner,
            wdk_sys::base::FILE_DEVICE_SCREEN => DeviceType::Screen,
            wdk_sys::base::FILE_DEVICE_SERENUM => DeviceType::Serenum,
            wdk_sys::base::FILE_DEVICE_SERIAL_MOUSE_PORT => DeviceType::SerialMousePort,
            wdk_sys::base::FILE_DEVICE_SERIAL_PORT => DeviceType::SerialPort,
            wdk_sys::base::FILE_DEVICE_SMARTCARD => DeviceType::Smartcard,
            wdk_sys::base::FILE_DEVICE_SMB => DeviceType::Smb,
            wdk_sys::base::FILE_DEVICE_SOUND => DeviceType::Sound,
            wdk_sys::base::FILE_DEVICE_STREAMS => DeviceType::Streams,
            wdk_sys::base::FILE_DEVICE_TAPE => DeviceType::Tape,
            wdk_sys::base::FILE_DEVICE_TAPE_FILE_SYSTEM => DeviceType::TapeFileSystem,
            wdk_sys::base::FILE_DEVICE_TERMSRV => DeviceType::Termsrv,
            wdk_sys::base::FILE_DEVICE_TRANSPORT => DeviceType::Transport,
            wdk_sys::base::FILE_DEVICE_UNKNOWN => DeviceType::Unknown,
            wdk_sys::base::FILE_DEVICE_VDM => DeviceType::Vdm,
            wdk_sys::base::FILE_DEVICE_VIDEO => DeviceType::Video,
            wdk_sys::base::FILE_DEVICE_VIRTUAL_DISK => DeviceType::VirtualDisk,
            wdk_sys::base::FILE_DEVICE_WAVE_IN => DeviceType::WaveIn,
            wdk_sys::base::FILE_DEVICE_WAVE_OUT => DeviceType::WaveOut,
            _ => DeviceType::Unknown,
        }
    }
}

#[repr(C)]
pub struct Operations {
    dispatch: Option<extern "C" fn(*mut DEVICE_OBJECT, *mut IRP, u8) -> NTSTATUS>,
    release: Option<extern "C" fn(*mut DEVICE_OBJECT)>,
}

pub struct Device {
    raw: *mut DEVICE_OBJECT,
}

impl Device {
    pub unsafe fn from_raw(raw: *mut DEVICE_OBJECT) -> Self {
        Self { raw }
    }

    pub unsafe fn as_raw(&self) -> *const DEVICE_OBJECT {
        self.raw as *const _
    }

    pub unsafe fn as_raw_mut(&self) -> *mut DEVICE_OBJECT {
        self.raw
    }

    pub fn into_raw(mut self) -> *mut DEVICE_OBJECT {
        core::mem::replace(&mut self.raw, core::ptr::null_mut())
    }

    pub fn extension(&self) -> &DeviceExtension {
        unsafe { &*((*self.raw).DeviceExtension as *const DeviceExtension) }
    }

    pub fn extension_mut(&self) -> &mut DeviceExtension {
        unsafe { &mut *((*self.raw).DeviceExtension as *mut DeviceExtension) }
    }

    pub fn device_type(&self) -> DeviceType {
        self.extension().device_type
    }

    pub fn vtable(&self) -> &Operations {
        unsafe { &*(self.extension().vtable as *const _) }
    }

    pub fn data<T: DeviceOperations>(&self) -> &T {
        unsafe { &*(self.extension().data as *const T) }
    }

    pub fn data_mut<T: DeviceOperations>(&self) -> &mut T {
        unsafe { &mut *(self.extension().data as *mut T) }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        if self.raw.is_null() {
            return;
        }

        unsafe {
            if let Some(release) = self.vtable().release {
                release(self.raw);
            }

            IoDeleteDevice(self.raw);
        }
    }
}

pub struct RequestError(pub Error, pub IoRequest);

pub enum Completion {
    Complete(u32, IoRequest),
}

pub trait DeviceOperations: Sync + Sized {
    fn create(&mut self, _device: &Device, request: IoRequest) -> Result<Completion, RequestError> {
        Ok(Completion::Complete(0, request))
    }

    fn close(&mut self, _device: &Device, request: IoRequest) -> Result<Completion, RequestError> {
        Ok(Completion::Complete(0, request))
    }

    fn cleanup(
        &mut self,
        _device: &Device,
        request: IoRequest,
    ) -> Result<Completion, RequestError> {
        Ok(Completion::Complete(0, request))
    }

    fn read(&mut self, _device: &Device, request: ReadRequest) -> Result<Completion, RequestError> {
        Ok(Completion::Complete(0, request.into()))
    }

    fn write(
        &mut self,
        _device: &Device,
        request: WriteRequest,
    ) -> Result<Completion, RequestError> {
        Ok(Completion::Complete(0, request.into()))
    }

    fn ioctl(
        &mut self,
        _device: &Device,
        request: IoControlRequest,
    ) -> Result<Completion, RequestError> {
        Ok(Completion::Complete(0, request.into()))
    }
}

extern "C" fn dispatch_callback<T: DeviceOperations>(
    device: *mut DEVICE_OBJECT,
    irp: *mut IRP,
    major: u8,
) -> NTSTATUS {
    let device = unsafe { Device::from_raw(device) };
    let data: &mut T = device.data_mut();
    let request = unsafe { IoRequest::from_raw(irp) };

    let result = match major as _ {
        IRP_MJ_CREATE => data.create(&device, request),
        IRP_MJ_CLOSE => data.close(&device, request),
        IRP_MJ_CLEANUP => data.cleanup(&device, request),
        IRP_MJ_READ => {
            let read_request = ReadRequest { inner: request };

            data.read(&device, read_request)
        }
        IRP_MJ_WRITE => {
            let write_request = WriteRequest { inner: request };

            data.write(&device, write_request)
        }
        IRP_MJ_DEVICE_CONTROL => {
            let control_request = IoControlRequest { inner: request };

            if device.device_type() == control_request.control_code().device_type() {
                data.ioctl(&device, control_request)
            } else {
                Err(RequestError(
                    Error::INVALID_PARAMETER,
                    control_request.into(),
                ))
            }
        }
        _ => Err(RequestError(Error::INVALID_PARAMETER, request)),
    };

    device.into_raw();

    match result {
        Ok(Completion::Complete(size, request)) => {
            request.complete(Ok(size));
            STATUS_SUCCESS
        }
        Err(RequestError(e, request)) => {
            let status = e.to_ntstatus();
            request.complete(Err(e));
            status
        }
    }
}

extern "C" fn release_callback<T: DeviceOperations>(device: *mut DEVICE_OBJECT) {
    unsafe {
        let extension = (*device).DeviceExtension as *mut DeviceExtension;

        let ptr = core::mem::replace(&mut (*extension).data, null_mut());
        Box::from_raw(ptr as *mut T);
    }
}

pub struct DeviceOperationsVtable<T>(PhantomData<T>);

impl<T: DeviceOperations> DeviceOperationsVtable<T> {
    pub const VTABLE: Operations = Operations {
        dispatch: Some(dispatch_callback::<T>),
        release: Some(release_callback::<T>),
    };
}

#[repr(C)]
pub struct DeviceExtension {
    pub vtable: *const Operations,
    pub data: *mut cty::c_void,
    pub device_type: DeviceType,
}

pub extern "C" fn dispatch_device(device: *mut DEVICE_OBJECT, irp: *mut IRP) -> NTSTATUS {
    let stack_location = unsafe { &*IoGetCurrentIrpStackLocation(irp) };
    let device = unsafe { Device::from_raw(device) };
    let vtable = device.vtable();

    match vtable.dispatch {
        Some(dispatch) => dispatch(device.into_raw(), irp, stack_location.MajorFunction),
        _ => {
            device.into_raw();
            STATUS_SUCCESS
        }
    }
}
