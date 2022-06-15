// Copyright (c) 2022 Connor Mellon
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{fs::File, fmt::{Display, self}, io::Write, error::Error};

use colored::Colorize;
use vulkanalia::vk::ErrorCode;
use winit::error::OsError;
use crate::ApplicationInfo;

pub enum LogLevel
{
	Trace = 0,
	Info = 1,
	Warning = 2,
	Error = 3,
	Critical = 4
}

impl Display for LogLevel
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self {
			LogLevel::Trace => write!(f, "Trace"),
			LogLevel::Info => write!(f, "Info"),
			LogLevel::Warning => write!(f, "Warning"),
			LogLevel::Error => write!(f, "Error"),
			LogLevel::Critical => write!(f, "Critical")
		}
	}
}


static mut LOGFILE: Option<File> = None;

pub fn init(app: &ApplicationInfo)
{
	let mut path = app.pref_path.clone();
	path.push("latest.log");
	let lf: Option<File> = File::create(&path).ok();
	unsafe { LOGFILE = lf; }
}

pub fn log(logger_name: &'static str, level: LogLevel, msg: String)
{
	let consopt = format!("[{logger_name}]: {msg}");
	let fopt = format!("[{logger_name}, {level}] {msg}");
	match level {
		LogLevel::Trace => write_file(fopt),
		LogLevel::Info => {
			println!("{}", format!("{consopt}").green());
			write_file(fopt);
		},
		LogLevel::Warning => {
			println!("{}", format!("{consopt}").yellow());
			write_file(fopt);
		},
		LogLevel::Error => {
			println!("{}", format!("{consopt}").red());
			write_file(fopt);
		},
		LogLevel::Critical => {
			println!("{}", format!("{consopt}").white().on_red());
			write_file(fopt);
		}
	}
}

fn write_file(str: String)
{
	unsafe {
		match &mut LOGFILE {
			Some(f) => { 
				f.write(str.as_bytes()).unwrap();
				f.write("\n".as_bytes()).unwrap();
			},
			None => {}
		}
	}
}


#[derive(Debug)]
pub enum IdioError
{
	PlatformError(String),
	VulkanError(String),
	Critical(&'static str)
}

impl std::error::Error for IdioError {}

impl fmt::Display for IdioError
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match self {
			IdioError::PlatformError(r) => write!(f, "Platform error: {r}"),
			IdioError::VulkanError(r) => write!(f, "Vulkan error: {r}"),
			IdioError::Critical(r) => write!(f, "Critical error: {r}"),		
		}
	}
}

impl From<OsError> for IdioError
{
	fn from(e: OsError) -> Self
	{
		IdioError::PlatformError(e.to_string())
	}
}

impl From<Box<(dyn Error + Sync + Send + 'static)>> for IdioError
{
	fn from(e: Box<(dyn Error + Sync + Send + 'static)>) -> Self
	{
		IdioError::PlatformError(e.to_string())
	}
}

impl From<ErrorCode> for IdioError
{
	fn from(e: ErrorCode) -> Self
	{
		IdioError::VulkanError(e.to_string())
	}
}