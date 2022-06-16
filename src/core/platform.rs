// Copyright (c) 2022 Connor Mellon
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(windows)]
pub(crate) unsafe fn enable_vtp()
{
    use winapi::{um::{consoleapi::{GetConsoleMode, SetConsoleMode}, winbase::STD_OUTPUT_HANDLE, processenv::GetStdHandle}};

	let stdout = GetStdHandle(STD_OUTPUT_HANDLE);
	let mut cmode: u32 = 0;
	GetConsoleMode(stdout, &mut cmode);
	SetConsoleMode(stdout, cmode | 0x0004);
}