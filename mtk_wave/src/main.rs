
#![macro_escape]

#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_variables)]
#[allow(dead_code)]

use vcd::{self, IdCode, SimulationCommand, TimescaleUnit, Value};

use std::{fs::File, io::Read};

use std::any::type_name;




use std::io;
use std::io::ErrorKind::InvalidInput;



fn type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}

use polars_core::prelude::*;
use polars_lazy::prelude::*;

fn main() -> Result<()> {
    let file = File::open("/home/mateusz/podklasa.vcd").unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    let mut parser = vcd::Parser::new(file);
    let header = parser.parse_header().unwrap();
    // let lol = header.timescale.unwrap().1;
    // println!("Hello, world! {:?} ", type_of(&lol));
    let clock = header
        .find_var(&["bench", "top", "clk"])
        .ok_or_else(|| io::Error::new(InvalidInput, "no wire clk"))?
        .code;

    let data = header
        .find_var(&["bench", "top", "a"])
        .ok_or_else(|| io::Error::new(InvalidInput, "no wire a"))?
        .code;

    let mut shift_reg = 0;
    let mut data_val = Value::X;
    let mut clock_val = Value::X;


    let df = df! {
        "column_a" => &[1, 2, 3, 4, 5],
        "column_b" => &["a", "b", "c", "d", "e"]
    }
    .unwrap();

    let new = df
        .lazy()
        // Note the reverse here!!
        .reverse()
        .with_column(
            // always rename a new column
            (col("column_a") * lit(10)).alias("new_column"),
        )
        .collect()
        .unwrap();

    assert!(new
        .column("new_column")
        .unwrap()
        .series_equal(&Series::new("new_column", &[50, 40, 30, 20, 10])));

    panic!("{}", "ka".parse::<IdCode>().unwrap());

    for command_result in parser {
        use vcd::Command::*;
        let command = command_result?;
        match command {
            ChangeScalar(i, v) if i == clock => {
                if clock_val == Value::V1 && v == Value::V0 {
                    // falling edge on clock
                    let shift_bit = match data_val {
                        Value::V1 => (1 << 31),
                        _ => 0,
                    };
                    shift_reg = (shift_reg >> 1) | shift_bit;
                }
                clock_val = v;
            }
            ChangeScalar(i, v) if i == data => {
                data_val = v;
            }
            _ => println!("WTF {:?}", command),
        }
        println!("A {:?}", clock_val);
    }
    Ok(())
}
