// Copyright (c) 2022 Connor Mellon
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use vulkanalia::{Instance, vk::{InstanceV1_0, self, HasBuilder}, Entry, window, loader::{LIBRARY, LibloadingLoader}};

use crate::{ApplicationInfo, logger::IdioError};

pub struct Context
{
	_entry: Entry,
	instance: Instance
}

impl Drop for Context
{
	fn drop(&mut self) 
	{
		unsafe {
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

		let exts = window::get_required_instance_extensions(&ai.main_window.as_ref().unwrap()
			.handle).iter().map(|e| e.as_ptr()).collect::<Vec<_>>();
		
		unsafe {
			let loader = LibloadingLoader::new(LIBRARY)
				.map_err(|e| IdioError::VulkanError(e.to_string()))?;
			let entry = Entry::new(loader)?;

			let ci = vk::InstanceCreateInfo::builder()
				.application_info(&appinfo)
				.enabled_extension_names(&exts);
			let instance = entry.create_instance(&ci, None)?;
				
			return Ok(Self {
				_entry: entry,
				instance: instance
			})
		}
	}
}