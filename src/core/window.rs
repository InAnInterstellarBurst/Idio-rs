// Copyright (c) 2022 Connor Mellon
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use winit::{window::{self, WindowBuilder, Fullscreen}, event_loop::EventLoop, dpi::PhysicalSize};

use crate::logger::IdioError;

#[derive(PartialEq, Eq)]
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

impl Default for WindowConfig
{
	fn default() -> Self
	{
		Self {
			borderless: true,
			allow_resize: true, 
			size_min: (1280, 720), 
			size_max: (0, 0), 
			title: "Idiot", 
			screen_mode: ScreenMode::Normal
		}
	}
}


pub struct Window
{
	pub handle: window::Window,
	pub id: window::WindowId
}

impl Window
{
	pub fn new(evt_loop: &EventLoop<()>, cfg: WindowConfig) -> Result<Self, IdioError>
	{
		let mut winbuilder = WindowBuilder::new()
			.with_title(cfg.title)
			.with_decorations(!cfg.borderless)
			.with_resizable(cfg.allow_resize);

		// Problem?
		let vmode = evt_loop.available_monitors().nth(0).unwrap()
			.video_modes().nth(0).unwrap();
		winbuilder = winbuilder.with_fullscreen(match cfg.screen_mode {
			ScreenMode::BorderlessFullscr => Some(Fullscreen::Borderless(None)),
			ScreenMode::ExclusiveFullscr => Some(Fullscreen::Exclusive(vmode)),
			_ => None
		});
		
		if cfg.size_max == (0, 0) {
			let sz = PhysicalSize::new(cfg.size_min.0, cfg.size_min.1);
			winbuilder = winbuilder.with_inner_size(sz);
		} else {
			let min = PhysicalSize::new(cfg.size_min.0, cfg.size_min.1);
			let max = PhysicalSize::new(cfg.size_max.0, cfg.size_max.1);
			
			winbuilder = winbuilder.with_min_inner_size(min);
			winbuilder = winbuilder.with_max_inner_size(max);
		}

		let handle = winbuilder.build(&evt_loop)?;
		return Ok(Self {
			id: handle.id(),
			handle: handle
		});
	}
}
