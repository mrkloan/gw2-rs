//! Guild Wars 2 Mumble Link API.

pub mod mumble;

use kernel32;
use winapi;

use std::{io, mem, ptr};
use libc::{c_void, wchar_t};

use self::mumble::*;

pub struct GW2 {
	last_tick: u32,
	handle: winapi::HANDLE,
	linked_mem: *mut c_void
}

impl GW2 {
	pub fn new() -> ::Result<Self> {
		let linked_mem_size = mem::size_of::<LinkedMem>();
		let mut shared_file: Vec<wchar_t> = "MumbleLink".chars()
			.map(|c| c as wchar_t)
			.collect();
		
		// NULL terminated string
		shared_file.push(0);
		
		unsafe {
			let handle = kernel32::CreateFileMappingW(
				winapi::shlobj::INVALID_HANDLE_VALUE,
				0 as *mut winapi::minwinbase::SECURITY_ATTRIBUTES,
				winapi::winnt::PAGE_READWRITE,
				0,
				linked_mem_size as u32,
				shared_file.as_ptr()
			);
			if handle.is_null() {
				return Err(io::Error::last_os_error().into());
			}
			
			let pointer = kernel32::MapViewOfFile(
				handle,
				winapi::FILE_MAP_ALL_ACCESS,
				0,
				0,
				linked_mem_size as u64,
			);
			if pointer.is_null() {
				kernel32::CloseHandle(handle);
				return Err(io::Error::last_os_error().into());
			}
			
			Ok(GW2 {
				last_tick: 0,
				handle: handle,
				linked_mem: pointer as *mut c_void
			})
		}
	}
	
	pub fn tick(&mut self) -> Option<LinkedMem> {
		let link = unsafe { ptr::read_volatile(self.linked_mem as *const LinkedMem) };
		
		match link.ui_tick() > self.last_tick {
			true => {
				self.last_tick = link.ui_tick();
				Some(link)
			},
			false => None
		}
	}
}

impl Drop for GW2 {
	fn drop(&mut self) {
		unsafe {
			kernel32::CloseHandle(self.handle);
		}
	}
}