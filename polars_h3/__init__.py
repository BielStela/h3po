import polars as pl
from polars.utils.udfs import _get_shared_lib_location

# boilerplate needed to inform polars of the location of binary wheel.
lib = _get_shared_lib_location(__file__)


@pl.api.register_expr_namespace("h3po")
class H3Polars:
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def cell_to_parent(self, resolution: int) -> pl.Expr:
        return self._expr._register_plugin(
            lib=lib,
            kwargs={
                "resolution": resolution
            },
            symbol="cell_to_parent",
            is_elementwise=True,
        )

    def cell_area(self) -> pl.Expr:
        return self._expr._register_plugin(
            lib=lib,
            symbol="cell_area",
            is_elementwise=True
        )
    
    def cell_area_aprox(self) -> pl.Expr:
        return self._expr._register_plugin(
            lib=lib,
            symbol="cell_area_aprox",
            is_elementwise=True
        )
