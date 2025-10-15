use candelatte::track::xlsx_to_gpx;
use std::{error::Error, fs::File, io::BufWriter, path::Path};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() <= 1 {
        panic!("Please provide an xlsx file to convert");
    }

    let in_path = Path::new(&args[1]);
    if in_path.extension().unwrap() != "xlsx" {
        panic!("Input file must be of xlsx type");
    }

    let mut out_path = in_path.to_path_buf();
    out_path.set_extension("gpx");

    xlsx_to_gpx(in_path, &out_path).unwrap();
}
