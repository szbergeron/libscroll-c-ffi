extern crate libscroll;
extern crate libc;

//use std::ptr;
use libscroll::Scrollview;
//use libc::*;

#[repr(C)]
pub enum Axis {
    Horizontal,
    Vertical,
}

/// Returns a handle on the scrollview
#[no_mangle]
pub unsafe extern fn alloc() -> *mut Scrollview {
    Box::into_raw(Box::new(Scrollview::new()))
}

#[no_mangle]
pub unsafe extern fn dealloc(scrollview: *mut Scrollview) {
    Box::from_raw(scrollview);
    // drops
}

// TODO: eval if params should be using libc:: types
#[no_mangle]
pub unsafe extern fn set_geometry(
    scrollview: *mut Scrollview,
    content_height: u64,
    content_width: u64,
    viewport_height: u64,
    viewport_width: u64,
) {
    scrollview
        .as_mut()
        .expect("ERROR: null/invalid ptr passed as scrollview handle for \"set_geometry\"")
        .set_geometry(
            content_height,
            content_width,
            viewport_height,
            viewport_width,
        );
}

#[no_mangle]
pub unsafe extern fn libscroll_animating(scrollview: *const Scrollview) -> bool {
    scrollview
        .as_ref()
        .expect("ERROR: null/invalid ptr passed as scrollview handle for \"animating\"")
        .animating()
}

#[no_mangle]
pub unsafe extern fn libscroll_step_frame(scrollview: *mut Scrollview) {
    scrollview
        .as_mut()
        .expect("ERROR: null/invalid ptr passed as scrollview handle for \"step_frame\"")
        .step_frame(None);
}

#[no_mangle]
pub unsafe extern fn libscroll_set_avg_frametime(scrollview: *mut Scrollview, milliseconds: f64) {
    scrollview
        .as_mut()
        .expect("ERROR: null/invalid ptr passed as scrollview handle for \"set_avg_frametime\"")
        .set_avg_frametime(milliseconds);
}

#[no_mangle]
pub unsafe extern fn libscroll_set_next_frame_predict(scrollview: *mut Scrollview, milliseconds: f64) {
    scrollview
        .as_mut()
        .expect("ERROR: null/invalid ptr passed as scrollview handle for \"set_next_frame_predict\"")
        .set_next_frame_predict(milliseconds);
}

#[no_mangle]
pub unsafe extern fn libscroll_get_position_absolute_x(scrollview: *const Scrollview) -> f64 {
    scrollview
        .as_ref()
        .expect("ERROR: null/invalid ptr passed as scrollview handle for \"get_position_absolute_x\"")
        .get_position_absolute()
        .x
}

#[no_mangle]
pub unsafe extern fn libscroll_get_position_absolute_y(scrollview: *const Scrollview) -> f64 {
    scrollview
        .as_ref()
        .expect("ERROR: null/invalid ptr passed as scrollview handle for \"get_position_absolute_y\"")
        .get_position_absolute()
        .y
}

/// Axis arg: 0 for horizontal, 1 for vertical. All other values result in panic
#[no_mangle]
pub unsafe extern fn libscroll_push_pan(scrollview: *mut Scrollview, axis: u32, amount: f64) {
    let axis = match axis {
        0 => libscroll::Axis::Horizontal,
        1 => libscroll::Axis::Vertical,
        _ => panic!("Bad axis marker. Expected 0 (horizontal) or 1 (vertical)"),
    };

    scrollview
        .as_mut()
        .expect("ERROR: null/invalid ptr passed as scrollview handle for \"push_pan\"")
        .push_pan(None, axis, amount);
}

/// Used to tell libscroll how much room is available on each edge to checkerboard.
///
/// Set to +infinity for no limit (default behavior if function not called)
///
/// NOTE: provisionally implemented here for api stability, awaiting
/// simulation support in libscroll proper
#[no_mangle]
pub unsafe extern fn libscroll_set_overscroll_allowance(
    scrollview: *mut Scrollview,
    top_dp: f64,
    bottom_dp: f64,
    left_dp: f64,
    right_dp: f64,
) {
    //
}
