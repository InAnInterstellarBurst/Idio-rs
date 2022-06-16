// Copyright (c) 2022 Connor Mellon
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashSet;

use vulkanalia::{Instance, vk::{InstanceV1_0, self, HasBuilder, EntryV1_0, ExtDebugUtilsExtension, DeviceV1_0}, Entry, window, loader::{LIBRARY, LibloadingLoader}, Device};

use crate::{ApplicationInfo, logger::IdioError, log_engine, LogLevel};

pub struct Context
{
	_entry: Entry,
	instance: Instance,
	debug_messenger: Option<vk::DebugUtilsMessengerEXT>,
	pdev: PhysicalDevice,
	device: Device,
	gfx_queue: vk::Queue
}

impl Drop for Context
{
	fn drop(&mut self) 
	{
		unsafe {
			self.device.destroy_device(None);
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

			let pdevs = instance.enumerate_physical_devices()?.into_iter().map(|p| PhysicalDevice::from(&instance, p));
			let pdev = match pdevs.max() {
				Some(d) => {
					if d.gfx_queue_idx.is_none() { // Ie: Best GPU lacks gfx => No GPU supports it
						return Err(IdioError::Critical("No GPU with graphics support found"));
					}

					d
				},
				None => {
					return Err(IdioError::Critical("No supported GPUs found"));
				}
			};

			let features = vk::PhysicalDeviceFeatures::builder();
			let queue_info = vk::DeviceQueueCreateInfo::builder()
				.queue_family_index(pdev.gfx_queue_idx.unwrap())
				.queue_priorities(&[1.0]);
			let qcis = &[queue_info];
			
			let dci = vk::DeviceCreateInfo::builder()
				.queue_create_infos(qcis)
				.enabled_layer_names(&vlayers)
				.enabled_features(&features);
			let device = instance.create_device(pdev.handle, &dci, None)?;
			let gfxq = device.get_device_queue(pdev.gfx_queue_idx.unwrap(), 0);

			log_engine!(LogLevel::Info, "Selected GPU {}", pdev.name);
			return Ok(Self {
				_entry: entry,
				pdev: pdev,
				instance: instance,
				debug_messenger: debug_messenger,
				device: device,
				gfx_queue: gfxq
			})
		}
	}
}

#[derive(PartialEq, Eq)]
struct PhysicalDevice
{
	handle: vk::PhysicalDevice,
	features: vk::PhysicalDeviceFeatures,
	name: String,
	device_type: vk::PhysicalDeviceType,
	gfx_queue_idx: Option<u32>
}

impl PhysicalDevice
{
	unsafe fn from(i: &Instance, hdl: vk::PhysicalDevice) -> Self
	{
		let qprops = i.get_physical_device_queue_family_properties(hdl);
		let gfxidx = qprops.iter()
		.position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
		.map(|i| i as u32);
		let props = i.get_physical_device_properties(hdl);

		Self { 
			handle: hdl,
			features: i.get_physical_device_features(hdl),
			name: props.device_name.to_string(),
			device_type: props.device_type,
			gfx_queue_idx: gfxidx
		}
	}
}

impl PartialOrd for PhysicalDevice
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> 
	{
		if other.device_type == vk::PhysicalDeviceType::CPU {
			return Some(std::cmp::Ordering::Greater);
		}
		
		if self.gfx_queue_idx.is_none() && other.gfx_queue_idx.is_some() {
			return Some(std::cmp::Ordering::Greater);
		} else if self.gfx_queue_idx.is_some() && other.gfx_queue_idx.is_none() {
			return Some(std::cmp::Ordering::Less);
		} else {
			if self.device_type == vk::PhysicalDeviceType::DISCRETE_GPU 
				&& other.device_type != vk::PhysicalDeviceType::DISCRETE_GPU {
				
				return Some(std::cmp::Ordering::Less);
			} else if self.device_type != vk::PhysicalDeviceType::DISCRETE_GPU 
				&& other.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
				
				return Some(std::cmp::Ordering::Greater);
			} else {
				return Some(std::cmp::Ordering::Equal);
			}
		}
	}
}

impl Ord for PhysicalDevice
{
	fn cmp(&self, other: &Self) -> std::cmp::Ordering 
	{
		self.partial_cmp(other).unwrap()
	}
}


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
