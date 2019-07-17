// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_camel_case_types)]
use libc::{self, c_int};
use std::io;
use std::mem;
use std::net::{TcpListener, TcpStream, UdpSocket};

// mod impls;

pub mod c {
    pub use libc::*;
    pub use ext::wasi_libc_extension::socklen_t;
    pub use ext::wasi_libc_extension::AF_INET;
    pub use ext::wasi_libc_extension::AF_INET6;
    pub use ext::wasi_libc_extension::SOCK_STREAM;
    pub use ext::wasi_libc_extension::SOL_SOCKET;
    pub use ext::wasi_libc_extension::in_addr;
    pub use ext::wasi_libc_extension::in6_addr;
    pub type in_port_t = u16;
    pub const SOCK_DGRAM: c_int = 0x00;
    pub const FD_CLOEXEC: c_int = 0x0;
    pub const SO_REUSEPORT: c_int = 0x0;
    pub type sa_family_t = u16;

    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
    }
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [c_char; 14],
    }

    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: in6_addr,
        pub sin6_scope_id: u32,
    }

    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [u8; 8],
    }

    extern {
        pub fn getsockname(socket: c_int, address: *mut sockaddr,
                       address_len: *mut socklen_t) -> c_int;
        pub fn connect(socket: c_int, address: *const sockaddr,
                   len: socklen_t) -> c_int;
        pub fn listen(socket: c_int, backlog: c_int) -> c_int;
        pub fn bind(socket: c_int, address: *const sockaddr,
                address_len: socklen_t) -> c_int;
        pub fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int;
        pub fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    }

    pub fn sockaddr_in_u32(sa: &sockaddr_in) -> u32 {
        ::ntoh((*sa).sin_addr.s_addr)
    }

    pub fn in_addr_to_u32(addr: &in_addr) -> u32 {
        ::ntoh(addr.s_addr)
    }
}

pub struct Socket {
    fd: c_int,
}

impl Socket {
    pub fn new(family: c_int, ty: c_int) -> io::Result<Socket> {
        unsafe {
            let fd = try!(::cvt(c::socket(family, ty, 0)));
            c::fcntl(fd, c::FD_CLOEXEC);
            Ok(Socket { fd: fd })
        }
    }

    pub fn raw(&self) -> libc::__wasi_fd_t {
        self.fd
    }

    fn into_fd(self) -> libc::__wasi_fd_t {
        let fd = self.fd;
        mem::forget(self);
        fd
    }

    pub fn into_tcp_listener(self) -> TcpListener {
        unsafe { TcpListener::from_raw_fd(self.into_fd()) }
    }

    pub fn into_tcp_stream(self) -> TcpStream {
        unsafe { TcpStream::from_raw_fd(self.into_fd()) }
    }

    pub fn into_udp_socket(self) -> UdpSocket {
        unsafe { UdpSocket::from_raw_fd(self.into_fd()) }
    }
}

impl ::FromInner for Socket {
    type Inner = libc::__wasi_fd_t;
    fn from_inner(fd: libc::__wasi_fd_t) -> Socket {
        Socket { fd: fd }
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe {
            // let _ = libc::close(self.fd);
        }
    }
}
