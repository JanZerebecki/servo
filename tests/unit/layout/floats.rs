/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use layout::floats::{Floats, FloatKind, PlacementInfo};
use style::logical_geometry::{LogicalRect, LogicalSize, WritingMode};
use app_units::Au;

#[test]
#[should_panic(expected = "Float position error")]
fn test_available_rect_translate_between_add_float() {
    let mode = WritingMode::empty();
    let mut f = Floats::new(mode);
    f.translate(LogicalSize::new(mode, Au::from_f64_px(-184.95), Au::from_f64_px(-15.95)));
    f.add_float(
        &PlacementInfo {
            size: LogicalSize::new(mode, Au::from_f64_px(199.6), Au::from_f64_px(318.3666666666667)),
            ceiling: Au::from_f64_px(0.0),
            max_inline_size: Au::from_f64_px(598.1),
            kind: FloatKind::Right,
        }
    );
    f.translate(LogicalSize::new(mode, Au::from_f64_px(0.0), Au::from_f64_px(-313.60)));
    f.add_float(
        &PlacementInfo {
            size: LogicalSize::new(mode, Au::from_f64_px(249.6), Au::from_f64_px(100.4)),
            ceiling: Au::from_f64_px(14.0),
            max_inline_size: Au::from_f64_px(598.1),
            kind: FloatKind::Left,
        }
    );
    // the first float still has the block size from before the second translate call, so this will trigger the assertion
    f.available_rect(Au::from_f64_px(0.0), Au::from_f64_px(16.283333333333335), Au::from_f64_px(598.1));
}

#[test]
fn test_available_rect_same_but_translate_before_add_float() {
    let mode = WritingMode::empty();
    let mut f = Floats::new(mode);
    f.translate(LogicalSize::new(mode, Au::from_f64_px(-184.95), Au::from_f64_px(-15.95)));
    f.translate(LogicalSize::new(mode, Au::from_f64_px(0.0), Au::from_f64_px(-313.60)));
    f.add_float(
        &PlacementInfo {
            size: LogicalSize::new(mode, Au::from_f64_px(199.6), Au::from_f64_px(318.3666666666667)),
            ceiling: Au::from_f64_px(0.0),
            max_inline_size: Au::from_f64_px(598.1),
            kind: FloatKind::Right,
        }
    );
    f.add_float(
        &PlacementInfo {
            size: LogicalSize::new(mode, Au::from_f64_px(249.6), Au::from_f64_px(100.4)),
            ceiling: Au::from_f64_px(14.0),
            max_inline_size: Au::from_f64_px(598.1),
            kind: FloatKind::Left,
        }
    );
    let result = f.available_rect(Au::from_f64_px(0.0), Au::from_f64_px(16.283333333333335), Au::from_f64_px(598.1));
    let expected = Some( LogicalRect::new(mode, Au::from_f64_px(249.6), Au::from_f64_px(14.0), Au::from_f64_px(148.9), Au::from_f64_px(100.4)));
    assert_eq!(result, expected);
}
