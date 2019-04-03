// ------------------------------------------------------------------------------
// Copyright 2019 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

#[derive(Debug)]
struct Dts1<T: ?Sized> {
    a: u8,
    b: T,
}

#[derive(Debug)]
#[repr(C)]
struct Dts2<T: ?Sized> {
    a: u8,
    b: T,
}

fn main() {
    let dts1_1 = Dts1 {a: 16, b: [0]};
    let dts1_2 = Dts1 {a: 16, b: [0, 1, 2]};
    let dts1_3 = Dts1 {a: 16, b: [0, 1, 2, 3, 4]};

    let dts2 = Dts2 {a: 16, b: [0, 1, 2, 3, 4]};

    println!("{:?} -> {} bytes", dts1_1, std::mem::size_of_val(&dts1_1));
    println!("{:?} -> {} bytes", dts1_2, std::mem::size_of_val(&dts1_2));
    println!("{:?} -> {} bytes", dts1_3, std::mem::size_of_val(&dts1_3));

    println!("{:?} -> {} bytes (C)", dts2, std::mem::size_of_val(&dts2));
}
