use crate::{
    error::Result,
    instrument::{
        info::{get_info, InstrumentInfo},
        Info,
    },
};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub mod async_stream;
pub mod connection_addr;
pub mod usbtmc;

/// Defines a marker trait that we will implement on each device interface
pub trait Interface: NonBlock + Read + Write + Info {}

/// This device can be set to be non-blocking. This is a requirement of an Interface
pub trait NonBlock {
    /// Set the interface not to block on reads to allow for polling.
    ///
    /// # Errors
    /// There may be errors that occur from the associated physical interface
    /// (e.g. LAN, USB).
    fn set_nonblocking(&mut self, enable: bool) -> Result<()>;
}

impl NonBlock for TcpStream {
    fn set_nonblocking(&mut self, enable: bool) -> crate::error::Result<()> {
        Ok(Self::set_nonblocking(self, enable)?)
    }
}

impl Info for TcpStream {
    // write all methods for Info trait here
    fn info(&mut self) -> Result<InstrumentInfo> {
        let ip_addr = self.peer_addr();
        if let Ok(ip_addr) = ip_addr {
            let ip_addr = ip_addr.ip();
            let uri = format!("http://{ip_addr}/lxi/identification");
            let resp = reqwest::blocking::get(uri);
            if let Ok(response) = resp {
                if let Ok(txt) = response.text() {
                    if let Ok(info) = InstrumentInfo::try_from(&txt) {
                        return Ok(info);
                    }
                }
            }
        }
        // if lxi page is not available, then get info from the instrument
        get_info(self)
    }
}

impl Interface for TcpStream {}
