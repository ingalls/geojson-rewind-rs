extern crate geojson;

use geojson::Feature;
use  std::f64::consts::PI;

const WGS84_RADIUS: f64 = 6378137.0;

pub fn rewind(feat: &mut Feature, outer: bool) {
    let geom = match &mut feat.geometry {
        None => { return },
        Some(geom) => geom
    };

    match &mut geom.value {
        geojson::Value::Polygon(poly) => {
            correct_rings(poly, outer)
        },
        geojson::Value::MultiPolygon(mpoly) => {
            for poly in mpoly {
                correct_rings(poly, outer)
            }
        },
        _ => { return }
    }
}

fn correct_rings(coords: &mut Vec<Vec<geojson::Position>>, outer: bool) {
    wind(&mut coords[0], outer);

    let mut i = 1;
    while i < coords.len() {
        wind(&mut coords[i], !outer);
        i = i + 1;
    }
}

fn wind(coords: &mut Vec<geojson::Position>, outer: bool) {
    // ---- mapbox/geojson-area ----
    //  Calculate the approximate area of the polygon were it projected onto
    //  the earth.  Note that this area will be positive if ring is oriented
    //  clockwise, otherwise it will be negative.
    //
    //  Reference:
    //  Robert. G. Chamberlain and William H. Duquette, "Some Algorithms for
    //    Polygons on a Sphere", JPL Publication 07-03, Jet Propulsion
    //    Laboratory, Pasadena, CA, June 2007
    //    http://trs-new.jpl.nasa.gov/dspace/handle/2014/40409
    let coords_len = coords.len() as u32;

    if coords_len <= 2 { return }

    let mut area: f64 = 0.0;

    for i in 0..coords_len {
        let mut lower_index: u32;
        let mut middle_index: u32;
        let mut upper_index: u32;

        if i == coords_len - 2 {
            lower_index = coords_len - 2;
            middle_index = coords_len - 1;
            upper_index = 0;
        } else if i == coords_len - 1 {
            lower_index = coords_len - 1;
            middle_index = 0;
            upper_index = 1;
        } else {
            lower_index = i;
            middle_index = i + 1;
            upper_index = i + 2;
        }

        let mut p1 = &coords[lower_index as usize];
        let mut p2 = &coords[middle_index as usize];
        let mut p3 = &coords[upper_index as usize];

        area = area + ((p3[0] * PI / 180.0) - (p1[0] * PI / 180.0)) * (p2[1] * PI / 180.0).sin();
    }

    area = area * WGS84_RADIUS * WGS84_RADIUS / 2.0;
    // ---- End of GeoJSON Area ----

    if area > 0.0 && !outer {
        coords.reverse();
    }
}
