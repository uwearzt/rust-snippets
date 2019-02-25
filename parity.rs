// ------------------------------------------------------------------------------
// Copyright 2019 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

// set parity bit 14 to even.
fn set_parity(par: u16) -> u16
{
	let mut x = par;

	x = (x & 0x00FF)^(x >> 8);
	x = (x & 0x000F)^(x >> 4);
	x = (x & 0x0003)^(x >> 2);
	x = (x & 0x0001)^(x >> 1);

	if x & 1 == 0x0001 {
		par | 0b0100_0000_0000_0000
	}
	else {
		par
	}

}

fn main()
{
	println!("{:016b} -> {:016b}", 0, set_parity(0));
	println!("{:016b} -> {:016b}", 1, set_parity(1));
	println!("{:016b} -> {:016b}", 6412, set_parity(6412));
	println!("{:016b} -> {:016b}", 6413, set_parity(6413));
	println!("{:016b} -> {:016b}", 15414, set_parity(15414));
	println!("{:016b} -> {:016b}", 15415, set_parity(15415));
}
