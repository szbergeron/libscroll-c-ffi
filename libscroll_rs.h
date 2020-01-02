#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

enum class libscroll_axis {
  Horizontal,
  Vertical,
};

struct libscroll_scrollview {
  uint8_t _private[0];
};

extern "C" {

/// Returns a handle on the scrollview
libscroll_scrollview *alloc();

void dealloc(libscroll_scrollview *scrollview);

bool libscroll_animating(const void *scrollview);

double libscroll_get_position_absolute_x(const libscroll_scrollview *scrollview);

double libscroll_get_position_absolute_y(const libscroll_scrollview *scrollview);

/// Axis arg: 0 for horizontal, 1 for vertical. All other values result in panic
void libscroll_push_pan(libscroll_scrollview *scrollview, libscroll_axis axis, double amount);

void libscroll_set_avg_frametime(libscroll_scrollview *scrollview, double milliseconds);

void libscroll_set_next_frame_predict(libscroll_scrollview *scrollview, double milliseconds);

/// Used to tell libscroll how much room is available on each edge to checkerboard.
///
/// Set to +infinity for no limit (default behavior if function not called)
///
/// NOTE: provisionally implemented here for api stability, awaiting
/// simulation support in libscroll proper
void libscroll_set_overscroll_allowance(libscroll_scrollview *scrollview,
                                        double top_dp,
                                        double bottom_dp,
                                        double left_dp,
                                        double right_dp);

void libscroll_step_frame(libscroll_scrollview *scrollview);

void set_geometry(libscroll_scrollview *scrollview,
                  uint64_t content_height,
                  uint64_t content_width,
                  uint64_t viewport_height,
                  uint64_t viewport_width);

} // extern "C"
