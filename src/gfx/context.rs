// Copyright (c) 2022 Connor Mellon
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashSet;

use vulkanalia::{Instance, vk::{InstanceV1_0, self, HasBuilder, EntryV1_0, ExtDebugUtilsExtension}, Entry, window, loader::{LIBRARY, LibloadingLoader}};

use crate::{ApplicationInfo, logger::IdioError, log_engine, LogLevel};

pub struct Context
{
	_entry: Entry,
	instance: Instance,
	debug_messenger: Option<vk::DebugUtilsMessengerEXT>,
	pdev: PhysicalDevice
}

impl Drop for Context
{
	fn drop(&mut self) 
	{
		unsafe {
			if cfg!(debug_assertions) {
				self.instance.destroy_debug_utils_messenger_ext(self.debug_messenger.unwrap(), None);
			}
			self.instance.destroy_instance(None);
		}
	}
}

impl Context
{
	pub fn new(ai: &ApplicationInfo) -> Result<Self, IdioError>
	{
		let appinfo = vk::ApplicationInfo::builder()
			.api_version(vk::make_version(1, 3, 0))
			.engine_name(b"Idio\0")
			.engine_version(vk::make_version(0, 0, 1))
			.application_name(ai.name.as_bytes())
			.application_version(vk::make_version(ai.ver.0, ai.ver.1, ai.ver.2));

		let mut exts = window::get_required_instance_extensions(
			&ai.main_window.as_ref().unwrap().handle)
			.iter().map(|e| e.as_ptr()).collect::<Vec<_>>();
		
		unsafe {
			let loader = LibloadingLoader::new(LIBRARY)
				.map_err(|e| IdioError::VulkanError(e.to_string()))?;
			let entry = Entry::new(loader)?;

			let mut vlayers: Vec<*const i8> = Vec::new();
			if cfg!(debug_assertions) {
				let lname = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");
				let avail_layers = entry.enumerate_instance_layer_properties()?
					.iter().map(|l| l.layer_name).collect::<HashSet<_>>();
					
				exts.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr());
				if avail_layers.contains(&lname) {
					vlayers = vec![lname.as_ptr()];
				} else {
					log_engine!(LogLevel::Warning, "Validation layer not supported, vulkan errors won't be reported");
				}
			}

			let mut di = vk::DebugUtilsMessengerCreateInfoEXT::builder()
				.message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
				.message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
				.user_callback(Some(debug_callback));
			let mut ci = vk::InstanceCreateInfo::builder()
				.application_info(&appinfo)
				.enabled_extension_names(&exts)
				.enabled_layer_names(&vlayers);
			if cfg!(debug_assertions) {
				ci = ci.push_next(&mut di);
			}

			let instance = entry.create_instance(&ci, None)?;
			let debug_messenger = if cfg!(debug_assertions) {
				Some(instance.create_debug_utils_messenger_ext(&di, None)?)
			} else {
				None
			};

			let pdevs = instance.enumerate_physical_devices()?.into_iter().map(|p| PhysicalDevice::from(p)).collect();

			return Ok(Self {
				_entry: entry,
				instance: instance,
				debug_messenger: debug_messenger
			})
		}
	}
}


struct PhysicalDevice
{
	handle: vk::PhysicalDevice
}

impl PhysicalDevice
{
	fn from(hdl: vk::PhysicalDevice) -> Self
	{
		Self { handle: hdl }
	}
}

#[cfg(debug_assertions)]
extern "system" fn debug_callback(
	severity: vk::DebugUtilsMessageSeverityFlagsEXT,
	type_: vk::DebugUtilsMessageTypeFlagsEXT,
	data: *const vk::DebugUtilsMessengerCallbackDataEXT,
	_: *mut std::os::raw::c_void) -> vk::Bool32
{
	use std::ffi::CStr;

	let data = unsafe { *data };
	let message = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();

	if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::ERROR {
		log_engine!(LogLevel::Error, "({:?}) {}", type_, message);
	} else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {
		log_engine!(LogLevel::Warning, "({:?}) {}", type_, message);
	} else {
		log_engine!(LogLevel::Trace, "({:?}) {}", type_, message);
	}

	vk::FALSE
}
