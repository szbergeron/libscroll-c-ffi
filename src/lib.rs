extern crate libscroll;
extern crate libc;

#[repr(C)]
pub enum libscroll_axis {
    Horizontal,
    Vertical,
}

#[repr(C)] pub struct libscroll_scrollview { _private: [u8; 0] }

/// Returns a handle on the scrollview
#[no_mangle]
pub unsafe extern fn alloc() -> *mut libscroll_scrollview {
    Box::into_raw(Box::new(libscroll::Scrollview::new())) as *mut libscroll_scrollview
}

#[no_mangle]
pub unsafe extern fn dealloc(scrollview: *mut libscroll_scrollview) {
    Box::from_raw(scrollview);
    // drops
}

// TODO: eval if params should be using libc:: types
#[no_mangle]
pub unsafe extern fn set_geometry(
    scrollview: *mut libscroll_scrollview,
    content_height: u64,
    content_width: u64,
    viewport_height: u64,
    viewport_width: u64,
) {
    let scrollview = scrollview as *mut libscroll::Scrollview;

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
pub unsafe extern fn libscroll_animating(scrollview: *const libc::c_void) -> bool {
    let scrollview = scrollview as *mut libscroll::Scrollview;

    scrollview
        .as_ref()
        .expect("ERROR: null/invalid ptr passed as scrollview handle for \"animating\"")
        .animating()
}

#[no_mangle]
pub unsafe extern fn libscroll_step_frame(scrollview: *mut libscroll_scrollview) {
    let scrollview = scrollview as *mut libscroll::Scrollview;

    scrollview
        .as_mut()
        .expect("ERROR: null/invalid ptr passed as scrollview handle for \"step_frame\"")
        .step_frame(None);
}

#[no_mangle]
pub unsafe extern fn libscroll_set_avg_frametime(scrollview: *mut libscroll_scrollview, milliseconds: f64) {
    let scrollview = scrollview as *mut libscroll::Scrollview;

    scrollview
        .as_mut()
        .expect("ERROR: null/invalid ptr passed as scrollview handle for \"set_avg_frametime\"")
        .set_avg_frametime(milliseconds);
}

#[no_mangle]
pub unsafe extern fn libscroll_set_next_frame_predict(scrollview: *mut libscroll_scrollview, milliseconds: f64) {
    let scrollview = scrollview as *mut libscroll::Scrollview;

    scrollview
        .as_mut()
        .expect("ERROR: null/invalid ptr passed as scrollview handle for \"set_next_frame_predict\"")
        .set_next_frame_predict(milliseconds);
}

#[no_mangle]
pub unsafe extern fn libscroll_get_position_absolute_x(scrollview: *const libscroll_scrollview) -> f64 {
    let scrollview = scrollview as *mut libscroll::Scrollview;

    scrollview
        .as_ref()
        .expect("ERROR: null/invalid ptr passed as scrollview handle for \"get_position_absolute_x\"")
        .get_position_absolute()
        .x
}

#[no_mangle]
pub unsafe extern fn libscroll_get_position_absolute_y(scrollview: *const libscroll_scrollview) -> f64 {
    let scrollview = scrollview as *mut libscroll::Scrollview;

    scrollview
        .as_ref()
        .expect("ERROR: null/invalid ptr passed as scrollview handle for \"get_position_absolute_y\"")
        .get_position_absolute()
        .y
}

/// Axis arg: 0 for horizontal, 1 for vertical. All other values result in panic
#[no_mangle]
pub unsafe extern fn libscroll_push_pan(scrollview: *mut libscroll_scrollview, axis: libscroll_axis, amount: f64) {
    let scrollview = scrollview as *mut libscroll::Scrollview;

    let axis = match axis {
        libscroll_axis::Horizontal => libscroll::Axis::Horizontal,
        libscroll_axis::Vertical => libscroll::Axis::Vertical,
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
    scrollview: *mut libscroll_scrollview,
    top_dp: f64,
    bottom_dp: f64,
    left_dp: f64,
    right_dp: f64,
) {
    let scrollview = scrollview as *mut libscroll::Scrollview;

    //
}
