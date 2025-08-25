use calamine::{RangeDeserializerBuilder, Reader, Xlsx, open_workbook};
use geo_types::{Point, coord};
use gpx::{Gpx, GpxVersion, Track, TrackSegment, Waypoint};
use std::{error::Error, fs::File, io::BufWriter, path::Path};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

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

    xlsx_to_gpx(&in_path, &out_path).unwrap();
}

fn xlsx_to_gpx(in_path: &Path, out_path: &Path) -> Result<(), Box<dyn Error>> {
    // prep xlsx
    let mut workbook: Xlsx<_> = open_workbook(in_path)?;
    let sheets = workbook.worksheets();
    let range = workbook.worksheet_range(&sheets[0].0).unwrap();

    let mut iter = RangeDeserializerBuilder::new().from_range(&range).unwrap();

    // prep gpx
    let track_segment = TrackSegment { points: vec![] };
    let track = Track {
        name: Some(sheets[0].0.clone()),
        comment: None,
        description: Some("Track from a Candela C8 hydrofoil boat".into()),
        source: Some("https://app.candela.com/trips".into()),
        links: vec![],
        type_: None,
        number: None,
        segments: vec![track_segment],
    };
    let mut gpx = Gpx {
        version: GpxVersion::Gpx11,
        creator: Some("candelatte".into()),
        metadata: None,
        waypoints: vec![],
        tracks: vec![track],
        routes: vec![],
    };

    // Create file at path
    let gpx_file = File::create(out_path).unwrap();
    let buf = BufWriter::new(gpx_file);

    // convert
    while let Some(result) = iter.next() {
        let (_, dts, _, _, lat, lon, _, _, _, knt): (
            String,
            String,
            String,
            String,
            f64,
            f64,
            String,
            String,
            f64,
            f64,
        ) = result.unwrap();

        let geo_coord = coord! { x: lon, y: lat };
        let geo_point: Point = geo_coord.into();
        let mut wp = Waypoint::new(geo_point);

        let mut dts = dts.replace(" ", "T");
        dts.truncate(23);
        dts.push_str("Z");
        let odt = OffsetDateTime::parse(&dts, &Rfc3339)?;
        let ti = gpx::Time::from(odt);

        wp.time = Some(ti);
        wp.speed = Some(knt * 1.852); // to km/h
        gpx.tracks[0].segments[0].points.push(wp);
    }

    gpx::write(&gpx, buf).unwrap();
    println!("file written to {:?}", out_path);

    Ok(())
}
