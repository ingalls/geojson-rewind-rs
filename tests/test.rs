extern crate geojson_rewind;
extern crate geojson;

#[cfg(test)]
mod test {
    use geojson_rewind::rewind;
    use geojson;

    #[test]
    fn flip() {
        let geom: geojson::GeoJson = r#"{
            "type": "Feature",
            "properties": {},
            "geometry": {
                "type": "Polygon",
                "coordinates": [
                    [ [100.0, 0.0], [101.0, 0.0], [101.0, 1.0], [100.0, 1.0], [100.0, 0.0] ],
                    [ [ 100.2, 0.2 ], [ 100.2, 0.8 ], [ 100.8, 0.8 ], [ 100.8, 0.2 ], [ 100.2, 0.2 ] ]
                ]
            }
        }"#.parse::<geojson::GeoJson>().unwrap();

        if let geojson::GeoJson::Feature(mut feat) = geom {
            rewind(&mut feat, false);

            assert_eq!(feat.geometry, Some(geojson::Geometry {
                bbox: None,
                foreign_members: None,
                value: geojson::Value::Polygon(vec![
                    vec![vec![100.0, 0.0], vec![101.0, 0.0], vec![101.0, 1.0], vec![100.0, 1.0], vec![100.0, 0.0]],
                    vec![vec![100.2, 0.2], vec![100.2, 0.8], vec![100.8, 0.8], vec![100.8, 0.2], vec![100.2, 0.2]]
                ])
            }));
        };
    }

    #[test]
    fn multipolygon() {
        let geom: geojson::GeoJson = r#"{
            "type": "Feature",
            "properties": {},
            "geometry": {
                "type": "MultiPolygon",
                "coordinates": [
                    [[[102.0, 2.0], [103.0, 2.0], [103.0, 3.0], [102.0, 3.0], [102.0, 2.0]]],
                    [[[100.0, 0.0], [101.0, 0.0], [101.0, 1.0], [100.0, 1.0], [100.0, 0.0]], [[100.2, 0.2], [100.8, 0.2], [100.8, 0.8], [100.2, 0.8], [100.2, 0.2]]]
                ]
            }
        }"#.parse::<geojson::GeoJson>().unwrap();

        if let geojson::GeoJson::Feature(mut feat) = geom {
            rewind(&mut feat, false);

            assert_eq!(feat.geometry, Some(geojson::Geometry {
                bbox: None,
                foreign_members: None,
                value: geojson::Value::MultiPolygon(vec![
                    vec![vec![vec![102.0,2.0],vec![103.0,2.0],vec![103.0,3.0],vec![102.0,3.0],vec![102.0,2.0]]],
                    vec![vec![vec![100.0,0.0],vec![101.0,0.0],vec![101.0,1.0],vec![100.0,1.0],vec![100.0,0.0]],vec![vec![100.2,0.2],vec![100.2,0.8],vec![100.8,0.8],vec![100.8,0.2],vec![100.2,0.2]]]
                ])
            }));
        };
    }
}
