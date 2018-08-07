<h1 align=center>GeoJSON-Rewind</h1>

<p align=center>Enforce polygon ring winding order for GeoJSON</p>

<p align="center">
  <a href="https://circleci.com/gh/ingalls/tree/master"><img src="https://circleci.com/gh/ingalls/tilecover/tree/master.svg?style=shield"/></a>
  <a href="https://crates.io/crates/geojson-rewind"><img src="https://img.shields.io/crates/v/geojson-rewind.svg"/></a>
</p>

<p align=center>
    Rust port of <a href="https://github.com/mapbox/geojson-rewind">@mapbox/geojson-rewind</a>
</p>

[![Build Status](https://travis-ci.org/mapbox/geojson-rewind.png)](https://travis-ci.org/mapbox/geojson-rewind)

The [GeoJSON](https://tools.ietf.org/html/rfc7946) specification is [picky about winding order](https://tools.ietf.org/html/rfc7946#section-3.1.6).

This helps you generate compliant Polygon and MultiPolygon geometries.

## API

`rewind(geojson: &mut geojson::Feature, outer: bool)`

Given a GeoJSON FeatureCollection, Feature, or Geometry, return a version
with inner and outer rings of different winding orders.

If `outer` is `true`, the outer ring is clockwise, otherwise it is counterclockwise.

