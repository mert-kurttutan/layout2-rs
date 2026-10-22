use layout::backends::svg::SVGWriter;
use layout::gv::{DotParser, GraphBuilder};
use wasm_bindgen::prelude::*;

/// Parse DOT source, lay out the graph, and return the rendered SVG.
///
/// This is the browser-facing equivalent of the CLI pipeline. It accepts and
/// returns strings so the JavaScript caller does not need filesystem access.
#[wasm_bindgen]
pub fn render_dot(
    source: &str,
    disable_optimizations: bool,
    disable_layout: bool,
) -> Result<String, JsValue> {
    let mut parser = DotParser::new(source);
    let ast = parser
        .process()
        .map_err(|error| JsValue::from_str(&error))?;

    let mut builder = GraphBuilder::new();
    builder.visit_graph(&ast);
    let mut graph = builder
        .try_get()
        .map_err(|error| JsValue::from_str(&error))?;

    let mut svg = SVGWriter::new();
    graph.do_it(false, disable_optimizations, disable_layout, &mut svg);
    Ok(svg.finalize())
}
