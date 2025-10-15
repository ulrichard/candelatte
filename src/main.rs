use calamine::{RangeDeserializerBuilder, Reader, Xlsx, open_workbook};
use geo_types::{Point, coord};
use gpx::{Gpx, GpxVersion, Link, Track, TrackSegment, Waypoint};
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

    xlsx_to_gpx(in_path, &out_path).unwrap();
}

fn xlsx_to_gpx(in_path: &Path, out_path: &Path) -> Result<(), Box<dyn Error>> {
    // read xlsx
    let mut workbook: Xlsx<_> = open_workbook(in_path)?;
    let sheets = workbook.worksheets();
    let range = workbook.worksheet_range(&sheets[0].0)?;

    // convert track points
    let trackpoints = RangeDeserializerBuilder::new()
        .from_range(&range)?
        .map(|record| {
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
            ) = record?;

            let geo_point: Point = coord! { x: lon, y: lat }.into();
            let mut wp = Waypoint::new(geo_point);

            let mut dts = dts.replace(" ", "T");
            dts.truncate(23);
            dts.push('Z');
            let odt = OffsetDateTime::parse(&dts, &Rfc3339)?;
            let ti = gpx::Time::from(odt);

            wp.time = Some(ti);
            wp.speed = Some(knt * 1.852); // to km/h

            Ok(wp)
        })
        .collect::<Result<Vec<Waypoint>, Box<dyn Error>>>()?;

    // write gpx
    let links = [
        "https://candela.com",
        "https://app.candela.com",
        "https://github.com/ulrichard/candelatte",
        "https://github.com/CandelaSpeedBoat",
    ]
    .iter()
    .map(|url| Link {
        href: (*url).into(),
        ..Default::default()
    })
    .collect::<Vec<_>>();
    let track = Track {
        name: Some(sheets[0].0.clone()),
        description: Some("Track from a Candela C8 electric hydrofoil boat".into()),
        source: Some("https://app.candela.com/trips".into()),
        links,
        segments: vec![TrackSegment {
            points: trackpoints,
        }],
        ..Default::default()
    };
    let gpx = Gpx {
        version: GpxVersion::Gpx11,
        creator: Some("candelatte".into()),
        metadata: None,
        waypoints: vec![],
        tracks: vec![track],
        routes: vec![],
    };

    // write to file
    let gpx_file = File::create(out_path)?;
    let buf = BufWriter::new(gpx_file);
    gpx::write(&gpx, buf)?;
    println!("file written to {:?}", out_path);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn get_test_track_file(track_name: &str) -> PathBuf {
        let test_dir = Path::new(file!()).canonicalize().unwrap();
        let test_dir = test_dir.as_path().parent().unwrap();
        let test_dir = test_dir.parent().unwrap();
        let track_file = test_dir.join("tests/data/".to_string() + track_name);
        assert!(
            track_file.exists(),
            "File not found: {}",
            track_file.to_str().unwrap()
        );
        track_file.to_path_buf()
    }

    #[test]
    fn convert_xlsx_to_gpx() {
        let track_file = get_test_track_file("boat_data_2025-06-29_19-41-12_trips_C8-S61.xlsx");
        let temp_dir = temp_dir::TempDir::new().unwrap();

        xlsx_to_gpx(&track_file, &temp_dir.child("boat_trip.gpx")).unwrap();
    }
}
