// Copyright (c) 2022 Connor Mellon
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use winit::{window::{self, WindowBuilder}, event_loop::EventLoop, dpi::PhysicalSize};

use crate::logger::IdioError;

pub enum ScreenMode
{
	Normal,
	BorderlessFullscr,
	ExclusiveFullscr
}

pub struct WindowConfig
{
	pub borderless: bool,
	pub allow_resize: bool,
	pub size_min: (u32, u32),
	pub size_max: (u32, u32),
	pub title: &'static str,
	pub screen_mode: ScreenMode
}

#[derive(Default)]
pub struct Window
{
	handle: window::Window
}

impl Window
{
	pub fn new(evt_loop: &EventLoop<()>, cfg: WindowConfig) -> Result<Self, IdioError>
	{
		let mut winbuilder = WindowBuilder::new()
			.with_title(cfg.title)
			.with_decorations(!cfg.borderless)
			.with_resizable(cfg.allow_resize);
		
		if cfg.size_max == (0, 0) {
			let sz = PhysicalSize::new(cfg.size_min.0, cfg.size_min.1);
			winbuilder = winbuilder.with_inner_size(sz);
		} else {
			let min = PhysicalSize::new(cfg.size_min.0, cfg.size_min.1);
			let max = PhysicalSize::new(cfg.size_max.0, cfg.size_max.1);
			
			winbuilder = winbuilder.with_min_inner_size(min);
			winbuilder = winbuilder.with_max_inner_size(max);
		}

		return Ok(Window {
			handle: winbuilder.build(&evt_loop)?
		});
	}
}
