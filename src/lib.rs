extern crate core;

use h3o::{CellIndex, Resolution};
use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct H3Kwargs {
    resolution: u8,
}

#[polars_expr(output_type = UInt64)]
fn cell_to_parent(inputs: &[Series], kwargs: H3Kwargs) -> PolarsResult<Series> {
    let ca = inputs[0].u64()?;
    let h3_resolution = Resolution::try_from(kwargs.resolution).unwrap();
    let out = ca.apply_values(|cell| {
        u64::from(
            CellIndex::try_from(cell)
                .unwrap()
                .parent(h3_resolution)
                .unwrap(),
        )
    });
    Ok(out.into_series())
}

#[polars_expr(output_type = Float64)]
fn cell_area(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].u64()?;
    let out: Float64Chunked =
        ca.apply_values_generic(|cell| CellIndex::try_from(cell).unwrap().area_km2());
    Ok(out.into_series())
}

#[polars_expr(output_type = Float64)]
fn cell_area_aprox(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].u64()?;
    let out: Float64Chunked =
        ca.apply_values_generic(|cell| CellIndex::try_from(cell).unwrap().resolution().area_km2());
    Ok(out.into_series())
}

fn cells_list(_input_fields: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "cells list",
        DataType::List(Box::new(DataType::UInt64)),
    ))
}

#[polars_expr(output_type_func = cells_list)]
fn uncompact(inputs: &[Series], kwargs: H3Kwargs) -> PolarsResult<Series> {
    let ca = inputs[0].u64()?;
    let resolution = Resolution::try_from(kwargs.resolution).unwrap();

    let mut builder: ListPrimitiveChunkedBuilder<UInt64Type> =
        ListPrimitiveChunkedBuilder::new("", ca.len(), ca.len(), DataType::UInt64);
    ca.for_each(|cell| match cell {
        Some(cell) => {
            let cell = CellIndex::try_from(cell).unwrap();
            let childs = cell.children(resolution).map(|c| u64::from(c)).collect::<Vec<_>>();
            builder.append_slice(childs.as_slice())
        }
        None => builder.append_null(),
    });
    let res = builder.finish();
    Ok(res.into_series())
}
