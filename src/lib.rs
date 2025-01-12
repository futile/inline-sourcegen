use std::ops::Bound;

use itertools::Itertools;

//@start gen_basic
// foobary!
//@end gen_basic

const START_MARKER: &str = "//@start";
const END_MARKER: &str = "//@end";

fn find_region<'a>(
    lines: impl Iterator<Item = &'a str> + Clone + std::fmt::Debug,
    region_name: &str,
) -> ((Bound<usize>, Bound<usize>), String) {
    let (start_pos, start_line) = lines
        .clone()
        .enumerate()
        .filter(|(_pos, v)| is_region_marker_line(v, START_MARKER, region_name))
        .exactly_one()
        .unwrap(); // TODO: this should be an error

    let (end_pos, _end_line) = lines
        .enumerate()
        .filter(|(_pos, v)| is_region_marker_line(v, END_MARKER, region_name))
        .exactly_one()
        .unwrap(); // TODO: this should be an error

    // TODO: this should be an error
    assert!(end_pos > start_pos);

    let (start_indent, _) = start_line.split_once(START_MARKER).unwrap(); // guaranteed to succeed because it's a region marker line
    // let end_indent = end_line.strip_suffix(&end_marker).unwrap();

    // TODO: this should be an error instead
    // assert_eq!(
    //     start_indent, end_indent,
    //     "mismatching start- and end-indentations"
    // );

    (
        (Bound::Excluded(start_pos), Bound::Excluded(end_pos)),
        start_indent.to_string(),
    )
}

fn is_region_marker_line(line: &str, marker: &str, region_name: &str) -> bool {
    let after_indent = line.trim_start();
    let Some(after_marker) = after_indent.strip_prefix(marker) else {
        return false;
    };
    let Some(after_space) = after_marker.strip_prefix(' ') else {
        return false;
    };
    after_space == region_name
}

fn should_write_codegen() -> bool {
    // TODO: Add check for some custom env-variable (for ci, etc.)
    true
}

#[cfg(test)]
mod test {
    use std::{borrow::Cow, ops::Deref as _};

    use super::*;

    #[test]
    fn sourcegen_gen_basic() {
        const REGION_NAME: &str = "gen_basic";
        const THIS_FILE: &str = file!();

        // TODO: use a write-lock so multiple tests that write to the same file don't race!
        let current_file = std::fs::read_to_string(THIS_FILE).unwrap();
        let mut current_lines = current_file.lines().map(Cow::from).collect::<Vec<_>>();
        let (region, indent) = find_region(current_lines.iter().map(|e| e.deref()), REGION_NAME);

        let generated_region: &[String] = &[format!("{indent}// foobary!")];

        if should_write_codegen() {
            let current_region = &current_lines[region];

            if current_region != generated_region {
                current_lines.splice(region, generated_region.iter().map_into());
                std::fs::write(THIS_FILE, current_lines.join("\n")).unwrap();

                panic!("sourcegen was required (file '{THIS_FILE}', region '{REGION_NAME}'), just re-run tests.");
            }
        }
    }
}