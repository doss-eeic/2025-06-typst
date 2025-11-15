use typst_library::{introspection::SplitLocator, layout::Axis};
use typst_utils::Numeric;

use super::*;

/// Turns the selected lines into frames.
#[typst_macros::time]
pub fn finalize(
    engine: &mut Engine,
    p: &Preparation,
    lines: &[Line],
    region: Size,
    expand: bool,
    locator: &mut SplitLocator<'_>,
) -> SourceResult<Fragment> {
    let (region_flow_length, region_cross_length) = match p.config.dir.axis() {
        Axis::X => (region.x, region.y),
        Axis::Y => (region.y, region.x),
    };

    // Determine the resulting length: Full flow dimension of the region if we should
    // expand or there's fractional spacing, fit-to-length otherwise.
    let length = if !region_flow_length.is_finite()
        || (!expand && lines.iter().all(|line| line.fr().is_zero()))
    {
        region_flow_length.min(
            p.config.hanging_indent
                + lines.iter().map(|line| line.length).max().unwrap_or_default(),
        )
    } else {
        region_flow_length
    };

    // Stack the lines into one frame per region.
    lines
        .iter()
        .map(|line| commit(engine, p, line, length, region_cross_length, locator))
        .collect::<SourceResult<_>>()
        .map(Fragment::frames)
}
