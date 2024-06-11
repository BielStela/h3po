from pathlib import Path

import polars as pl
from polars.plugins import register_plugin_function
from polars.type_aliases import IntoExpr


@pl.api.register_expr_namespace("h3po")
class H3Polars:
    def __init__(self, expr: IntoExpr):
        self._expr = expr

    def cell_to_parent(self, resolution: int) -> pl.Expr:
        return register_plugin_function(
            plugin_path=Path(__file__).parent,
            args=self._expr,
            kwargs={"resolution": resolution},
            function_name="cell_to_parent",
            is_elementwise=True,
        )

    def cell_area(self) -> pl.Expr:
        return register_plugin_function(
            plugin_path=Path(__file__).parent,
            args=self._expr,
            function_name="cell_area",
            is_elementwise=True,
        )

    def cell_area_aprox(self) -> pl.Expr:
        return register_plugin_function(
            plugin_path=Path(__file__).parent,
            args=self._expr,
            function_name="cell_area_aprox",
            is_elementwise=True,
        )

    def uncompact(self, resolution: int) -> pl.Expr:
        return register_plugin_function(
            plugin_path=Path(__file__).parent,
            args=self._expr,
            kwargs={"resolution": resolution},
            function_name="uncompact",
            is_elementwise=True,
        )
