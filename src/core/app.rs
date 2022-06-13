// Copyright (c) 2022 Connor Mellon
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fs;
use std::path::PathBuf;

use winit::{event_loop::{EventLoop, ControlFlow}, platform::run_return::EventLoopExtRunReturn, event::WindowEvent};

use crate::{logger::{self, IdioError}, WindowConfig, Window};

#[derive(Default)]
pub struct ApplicationInfo
{
	pub name: &'static str,
	pub pref_path: PathBuf,
	pub main_window: Option<Window>
}

pub trait Application
{
	fn init(&mut self, info: &ApplicationInfo);
	fn tick(&self);
	fn deinit(&mut self);
	fn event_handler(&mut self, evt: Event);
}

pub struct Event
{

}

pub fn run<T: Application>(name: &'static str, mut app: T, wincfg: WindowConfig) -> Result<(), IdioError>
{
	let mut datapath = match dirs::data_local_dir() {
		Some(d) => d,
		None => PathBuf::from("./")
	};

	let datasubdir: PathBuf = ["idio".to_string(), name.to_lowercase()].iter().collect();
	datapath.push(datasubdir);
	if fs::create_dir_all(&datapath).is_err() {
		println!("Preinit warning: Using ./ as prefpath");
		datapath = PathBuf::from("./");
	}

	let mut evt_loop = EventLoop::new();

	let ai = ApplicationInfo {
		name: name,
		pref_path: datapath,
		main_window: Some(Window::new(&evt_loop, wincfg)?)
	};

	let mut running = true;
	logger::init(&ai);
	app.init(&ai);
	while running {
		app.tick();
		
		evt_loop.run_return(|event, _, ctrl_flow| {
			*ctrl_flow = ControlFlow::Wait;
			match event {
				winit::event::Event::WindowEvent { 
					window_id, 
					event: WindowEvent::CloseRequested 
				} => {
					if window_id == ai.main_window.as_ref().unwrap().id {
						running = false;
						*ctrl_flow = ControlFlow::Exit;
					}
				},
				_ => {}
			}
		});
	}

	app.deinit();
	return Ok(());
}