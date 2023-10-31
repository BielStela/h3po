extern crate core;

use core::f64;

use h3o::{CellIndex, Resolution};
use polars::prelude::{ChunkApply, Float64Chunked, IntoSeries, PolarsResult, Series};
use pyo3::impl_::wrap::SomeWrap;
// use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MyKwargs {
    resolution: u8,
}

#[polars_expr(output_type = UInt64)]
fn cell_to_parent(inputs: &[Series], kwargs: MyKwargs) -> PolarsResult<Series> {
    let ca = inputs[0].u64()?;
    let h3_resolution = Resolution::try_from(kwargs.resolution).unwrap();
    let out = ca.apply_values(
        |cell| {
            u64::from(CellIndex::try_from(cell).unwrap().parent(h3_resolution).unwrap())
        });
    Ok(out.into_series())
}


#[polars_expr(output_type = Float64)]
fn cell_area(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].u64()?;
    let out: Float64Chunked = ca.apply_values_generic(
        |cell|  {
            CellIndex::try_from(cell).unwrap().area_km2()
        });
    Ok(out.into_series())
}


#[polars_expr(output_type = Float64)]
fn cell_area_aprox(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].u64()?;
    let out: Float64Chunked = ca.apply_values_generic(
        |cell| CellIndex::try_from(cell).unwrap().resolution().area_km2()
    );
    Ok(out.into_series())
}