// Copyright (c) 2022 Connor Mellon
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use idio::WindowConfig;
use winit::event_loop::EventLoop;

#[derive(Default)]
struct Test;

impl idio::Application for Test
{
	fn init(&mut self, _: &idio::ApplicationInfo, _evt_loop: &EventLoop<()>)
	{
	}

	fn tick(&self)
	{
	}

	fn deinit(&mut self)
	{
	}

	fn event_handler(&mut self, _: idio::Event)
	{
	}
}

fn main()
{
	let wincfg = WindowConfig {
		title: "Hello",
		borderless: false,
		allow_resize: true,
		.. Default::default()
	};

	match idio::run("TestApp", Test::default(), wincfg) {
		Err(e) => idio::log_game!(idio::LogLevel::Critical, "{e}"),
		_ => {}
	}
}
