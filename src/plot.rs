//! # Plot module
//!
//! This module defines the `Plot` struct, which is used to create a 2D plot that will
//! contain all other objects that can be created using this library.
use crate::{get_x_axis_from_index, get_x_axis_index, get_y_axis_from_index, get_y_axis_index, Axis, Context, PlotLocation, PlotUi, NUMBER_OF_X_AXES, NUMBER_OF_Y_AXES};
use bitflags::bitflags;
pub use imgui::Condition;
use implot_sys as sys;
use std::ffi::CString;
use std::os::raw::c_char;
use std::{
    cell::RefCell, 
    rc::Rc
};
pub use sys::{
    ImPlotRange,
    ImVec2,
};

const DEFAULT_PLOT_SIZE_X: f32 = 400.0;
const DEFAULT_PLOT_SIZE_Y: f32 = 400.0;

bitflags! {
    /// Flags for customizing plot behavior and interaction. Documentation copied from implot.h for
    /// convenience. ImPlot itself also has a "CanvasOnly" flag, which can be emulated here with
    /// the combination of `NO_LEGEND`, `NO_MENUS`, `NO_BOX_SELECT` and `NO_MOUSE_POSITION`.
    #[repr(transparent)]
    pub struct PlotFlags: u32 {
        /// "Default" according to original docs
        const NONE = sys::ImPlotFlags__ImPlotFlags_None as u32;
        /// the plot title will not be displayed (titles are also hidden if preceeded by double hashes, e.g. "##MyPlot")
        const NO_TITLE = sys::ImPlotFlags__ImPlotFlags_NoTitle as u32;
        /// Plot items will not be highlighted when their legend entry is hovered
        const NO_LEGEND = sys::ImPlotFlags__ImPlotFlags_NoLegend as u32;
        /// the mouse position, in plot coordinates, will not be displayed inside of the plot
        const NO_MOUSE_TEXT = sys::ImPlotFlags__ImPlotFlags_NoMouseText as u32;
        /// the user will not be able to interact with the plot
        const NO_INPUTS = sys::ImPlotFlags__ImPlotFlags_NoInputs as u32;
        /// The user will not be able to open context menus with double-right click
        const NO_MENUS = sys::ImPlotFlags__ImPlotFlags_NoMenus as u32;
        /// The user will not be able to box-select with right-mouse
        const NO_BOX_SELECT = sys::ImPlotFlags__ImPlotFlags_NoBoxSelect as u32;
        /// The mouse position, in plot coordinates, will not be displayed
        /// the ImGui frame will not be rendered
        const NO_FRAME = sys::ImPlotFlags__ImPlotFlags_NoFrame as u32;
        /// x and y axes pairs will be constrained to have the same units/pixel
        const EQUAL = sys::ImPlotFlags__ImPlotFlags_Equal as u32;
        /// the default mouse cursor will be replaced with a crosshair when hovered
        const CROSSHAIRS = sys::ImPlotFlags__ImPlotFlags_Equal as u32;
    }
}

bitflags! {
    /// Axis flags. Documentation copied from implot.h for convenience. ImPlot itself also
    /// has `Lock`, which combines `LOCK_MIN` and `LOCK_MAX`, and `NoDecorations`, which combines
    /// `NO_GRID_LINES`, `NO_TICK_MARKS` and `NO_TICK_LABELS`.
    #[repr(transparent)]
    pub struct AxisFlags: u32 {
        /// "Default" according to original docs
        const NONE = sys::ImPlotAxisFlags__ImPlotAxisFlags_None as u32;
        /// the axis label will not be displayed (axis labels are also hidden if the supplied string name is nullptr)
        const NO_LABEL = sys::ImPlotAxisFlags__ImPlotAxisFlags_NoLabel as u32;
        /// Grid lines will not be displayed
        const NO_GRID_LINES = sys::ImPlotAxisFlags__ImPlotAxisFlags_NoGridLines as u32;
        /// Tick marks will not be displayed
        const NO_TICK_MARKS = sys::ImPlotAxisFlags__ImPlotAxisFlags_NoTickMarks as u32;
        /// Text labels will not be displayed
        const NO_TICK_LABELS = sys::ImPlotAxisFlags__ImPlotAxisFlags_NoTickLabels as u32;
        /// axis will not be initially fit to data extents on the first rendered frame
        const NO_INITIAL_FIT = sys::ImPlotAxisFlags__ImPlotAxisFlags_NoTickLabels as u32;
        /// the user will not be able to open context menus with right-click
        const NO_MENUS = sys::ImPlotAxisFlags__ImPlotAxisFlags_NoMenus as u32;
        /// the user will not be able to switch the axis side by dragging it
        const NO_SIDE_SWITCH = sys::ImPlotAxisFlags__ImPlotAxisFlags_NoSideSwitch as u32;
        /// the axis will not have its background highlighted when hovered or held
        const NO_HIGHLIGHT = sys::ImPlotAxisFlags__ImPlotAxisFlags_NoHighlight as u32;
        const OPPOSITE = sys::ImPlotAxisFlags__ImPlotAxisFlags_Opposite as u32;
        const FOREGROUND = sys::ImPlotAxisFlags__ImPlotAxisFlags_Foreground as u32;
        /// The axis will be inverted
        const INVERT = sys::ImPlotAxisFlags__ImPlotAxisFlags_Invert as u32;
        const AUTO_FIT = sys::ImPlotAxisFlags__ImPlotAxisFlags_AutoFit as u32;
        const RANGE_FIT = sys::ImPlotAxisFlags__ImPlotAxisFlags_RangeFit as u32;
        const PAN_STRETCH = sys::ImPlotAxisFlags__ImPlotAxisFlags_PanStretch as u32;
        /// The axis minimum value will be locked when panning/zooming
        const LOCK_MIN = sys::ImPlotAxisFlags__ImPlotAxisFlags_LockMin as u32;
        /// The axis maximum value will be locked when panning/zooming
        const LOCK_MAX = sys::ImPlotAxisFlags__ImPlotAxisFlags_LockMax as u32;
        const LOCK = sys::ImPlotAxisFlags__ImPlotAxisFlags_Lock as u32;
        const NO_DECORATIONS = sys::ImPlotAxisFlags__ImPlotAxisFlags_NoDecorations as u32;
        const AUX_DEFAULT = sys::ImPlotAxisFlags__ImPlotAxisFlags_AuxDefault as u32;
    }
}

bitflags! {
    #[repr(transparent)]
    pub struct LineFlags: u32 {
        const NONE = sys::ImPlotLineFlags__ImPlotLineFlags_None        as u32;       // default
        const SEGMENTS = sys::ImPlotLineFlags__ImPlotLineFlags_Segments    as u32; // a line segment will be rendered from every two consecutive points
        const LOOP = sys::ImPlotLineFlags__ImPlotLineFlags_Loop        as u32; // the last and first point will be connected to form a closed loop
        const SKIP_NAN = sys::ImPlotLineFlags__ImPlotLineFlags_SkipNaN     as u32; // NaNs values will be skipped instead of rendered as missing data
        const NO_CLIP = sys::ImPlotLineFlags__ImPlotLineFlags_NoClip      as u32; // markers (if displayed) on the edge of a plot will not be clipped
        const SHADED = sys::ImPlotLineFlags__ImPlotLineFlags_Shaded      as u32; // a filled region between the line and horizontal origin will be rendered; use PlotShaded for more advanced cases
    }
}

bitflags! {
    #[repr(transparent)]
    pub struct StairsFlags: u32 {
        const NONE = sys::ImPlotStairsFlags__ImPlotStairsFlags_None     as u32;       // default
        const PRE_STEP = sys::ImPlotStairsFlags__ImPlotStairsFlags_PreStep  as u32; // the y value is continued constantly to the left from every x position, i.e. the interval (x[i-1], x[i]] has the value y[i]
        const SHADED = sys::ImPlotStairsFlags__ImPlotStairsFlags_Shaded   as u32; // a filled region between the stairs and horizontal origin will be rendered; use PlotShaded for more advanced cases
    }
}

bitflags! {
    #[repr(transparent)]
    pub struct ScatterFlags: u32 {
        const NONE = sys::ImPlotScatterFlags__ImPlotScatterFlags_None   as u32;       // default
        const NO_CLIP = sys::ImPlotScatterFlags__ImPlotScatterFlags_NoClip as u32; // markers on the edge of a plot will not be clipped
    }
}

bitflags! {
    #[repr(transparent)]
    pub struct BarsFlags: u32 {
        const NONE = sys::ImPlotBarsFlags__ImPlotBarsFlags_None   as u32;       // default
        const HORIZONTAL = sys::ImPlotBarsFlags__ImPlotBarsFlags_Horizontal as u32; // markers on the edge of a plot will not be clipped
    }
}

bitflags! {
    #[repr(transparent)]
    pub struct TextFlags: u32 {
        const NONE = sys::ImPlotTextFlags__ImPlotTextFlags_None   as u32;       // default
        const VERTICAL = sys::ImPlotTextFlags__ImPlotTextFlags_Vertical as u32; // text will be rendered vertically
    }
}

bitflags! {
    #[repr(transparent)]
    pub struct HeatmapFlags: u32 {
        const NONE = sys::ImPlotHeatmapFlags__ImPlotHeatmapFlags_None   as u32;       // default
        const COL_MAJOR = sys::ImPlotHeatmapFlags__ImPlotHeatmapFlags_ColMajor as u32; // data will be read in column major order
    }
}

bitflags! {
    #[repr(transparent)]
    pub struct StemsFlags: u32 {
        const NONE = sys::ImPlotStemsFlags__ImPlotStemsFlags_None   as u32;       // default
        const HORIZONTAL = sys::ImPlotStemsFlags__ImPlotStemsFlags_Horizontal as u32; // lines will be rendered horizontally on the current y-axis
    }
}

bitflags! {
    #[repr(transparent)]
    pub struct LegendFlags: u32 {
        const NONE = sys::ImPlotLegendFlags__ImPlotLegendFlags_None            as u32;      // default
        const NO_BUTTONS = sys::ImPlotLegendFlags__ImPlotLegendFlags_NoButtons       as u32; // legend icons will not function as hide/show buttons
        const NO_HIGHLIGHT_ITEM = sys::ImPlotLegendFlags__ImPlotLegendFlags_NoHighlightItem as u32; // plot items will not be highlighted when their legend entry is hovered
        const NO_HIGHLIGHT_AXIS = sys::ImPlotLegendFlags__ImPlotLegendFlags_NoHighlightAxis as u32; // axes will not be highlighted when legend entries are hovered (only relevant if x/y-axis count > 1)
        const NO_MENUS = sys::ImPlotLegendFlags__ImPlotLegendFlags_NoMenus         as u32; // the user will not be able to open context menus with right-click
        const OUTSIDE = sys::ImPlotLegendFlags__ImPlotLegendFlags_Outside         as u32; // legend will be rendered outside of the plot area
        const HORIZONTAL = sys::ImPlotLegendFlags__ImPlotLegendFlags_Horizontal      as u32; // legend entries will be displayed horizontally
        const SORT = sys::ImPlotLegendFlags__ImPlotLegendFlags_Sort            as u32; // legend entries will be displayed in alphabetical order
    }
}

/// Internally-used struct for storing axis limits
#[derive(Clone)]
enum AxisLimitSpecification {
    /// Direct limits, specified as values
    Single(ImPlotRange, Condition),
    /// Limits that are linked to limits of other plots (via clones of the same Rc)
    Linked(Rc<RefCell<ImPlotRange>>),
}

/// Struct to represent an ImPlot. This is the main construct used to contain all kinds of plots in ImPlot.
///
/// `Plot` is to be used (within an imgui window) with the following pattern:
/// ```no_run
/// # use implot;
/// let plotting_context = implot::Context::create();
/// let plot_ui = plotting_context.get_plot_ui();
/// implot::Plot::new("my title")
///     .size([300.0, 200.0]) // other things such as .x_label("some_label") can be added too
///     .build(&plot_ui, || {
///         // Do things such as plotting lines
///     });
///
/// ```
/// (If you are coming from the C++ implementation or the C bindings: build() calls both
/// begin() and end() internally)
pub struct Plot {
    /// Title of the plot, shown on top. Stored as CString because that's what we'll use
    /// afterwards, and this ensures the CString itself will stay alive long enough for the plot.
    title: CString,
    /// Size of the plot in [x, y] direction, in the same units imgui uses.
    size: [f32; 2],
    /// Label of the x axis, shown on the bottom. Stored as CString because that's what we'll use
    /// afterwards, and this ensures the CString itself will stay alive long enough for the plot.
    x_label: CString,
    /// Label of the y axis, shown on the left. Stored as CString because that's what we'll use
    /// afterwards, and this ensures the CString itself will stay alive long enough for the plot.
    y_label: CString,
    /// X axis limits, if present
    x_limits: [Option<AxisLimitSpecification>; NUMBER_OF_X_AXES],
    /// Y axis limits, if present
    y_limits: [Option<AxisLimitSpecification>; NUMBER_OF_Y_AXES],
    /// Positions for custom X axis ticks, if any
    x_tick_positions: [Option<Vec<f64>>; NUMBER_OF_X_AXES],
    /// Labels for custom X axis ticks, if any. I'd prefer to store these together
    /// with the positions in one vector of an algebraic data type, but this would mean extra
    /// copies when it comes time to draw the plot because the C++ library expects separate lists.
    /// The data is stored as CStrings because those are null-terminated, and since we have to
    /// convert to null-terminated data anyway, we may as well do that directly instead of cloning
    /// Strings and converting them afterwards.
    x_tick_labels: [Option<Vec<CString>>; NUMBER_OF_X_AXES],
    /// Whether to also show the default X ticks when showing custom ticks or not
    show_x_default_ticks: [bool; NUMBER_OF_X_AXES],
    /// Positions for custom Y axis ticks, if any
    y_tick_positions: [Option<Vec<f64>>; NUMBER_OF_Y_AXES],
    /// Labels for custom Y axis ticks, if any. I'd prefer to store these together
    /// with the positions in one vector of an algebraic data type, but this would mean extra
    /// copies when it comes time to draw the plot because the C++ library expects separate lists.
    /// The data is stored as CStrings because those are null-terminated, and since we have to
    /// convert to null-terminated data anyway, we may as well do that directly instead of cloning
    /// Strings and converting them afterwards.
    y_tick_labels: [Option<Vec<CString>>; NUMBER_OF_Y_AXES],
    /// Whether to also show the default Y ticks when showing custom ticks or not
    show_y_default_ticks: [bool; NUMBER_OF_Y_AXES],
    /// Configuration for the legend, if specified. The tuple contains location, orientation
    /// and a boolean (true means legend is outside of plot, false means within). If nothing
    /// is set, implot's defaults are used. Note also  that if these are set, then implot's
    /// interactive legend configuration does not work because it is overridden by the settings
    /// here.
    legend_configuration: Option<(PlotLocation, LegendFlags)>,
    /// Flags relating to the plot TODO(4bb4) make those into bitflags
    plot_flags: PlotFlags,
    /// Flags relating to the X axis of the plot TODO(4bb4) make those into bitflags
    x_flags: [AxisFlags; NUMBER_OF_X_AXES],
    /// Flags relating to the each of the Y axes of the plot TODO(4bb4) make those into bitflags
    y_flags: [AxisFlags; NUMBER_OF_Y_AXES],
}

impl Plot {
    /// Create a new plot with some defaults set. Does not draw anything yet.
    /// Note that this uses antialiasing by default, unlike the C++ API. If you are seeing
    /// artifacts or weird rendering, try disabling it.
    ///
    /// # Panics
    /// Will panic if the title string contains internal null bytes.
    pub fn new(title: &str) -> Self {
        // Needed for initialization, see https://github.com/rust-lang/rust/issues/49147
        const POS_NONE: Option<Vec<f64>> = None;
        const TICK_NONE: Option<Vec<CString>> = None;

        // TODO(4bb4) question these defaults, maybe remove some of them
        Self {
            title: CString::new(title)
                .unwrap_or_else(|_| panic!("String contains internal null bytes: {}", title)),
            size: [DEFAULT_PLOT_SIZE_X, DEFAULT_PLOT_SIZE_Y],
            x_label: CString::new("").unwrap(),
            y_label: CString::new("").unwrap(),
            x_limits: Default::default(),
            y_limits: Default::default(),
            x_tick_positions: [POS_NONE; NUMBER_OF_X_AXES],
            x_tick_labels: [TICK_NONE; NUMBER_OF_X_AXES],
            show_x_default_ticks: [false; NUMBER_OF_X_AXES],
            y_tick_positions: [POS_NONE; NUMBER_OF_Y_AXES],
            y_tick_labels: [TICK_NONE; NUMBER_OF_Y_AXES],
            show_y_default_ticks: [false; NUMBER_OF_Y_AXES],
            legend_configuration: None,
            plot_flags: PlotFlags::empty(),
            x_flags: [AxisFlags::empty(); NUMBER_OF_X_AXES],
            y_flags: [AxisFlags::empty(); NUMBER_OF_Y_AXES],
        }
    }

    /// Sets the plot size, given as [size_x, size_y]. Units are the same as
    /// what imgui uses. TODO(4bb4) ... which is? I'm not sure it's pixels
    #[inline]
    pub fn size(mut self, size: [f32; 2]) -> Self {
        self.size = size;
        self
    }

    /// Set the x label of the plot
    ///
    /// # Panics
    /// Will panic if the label string contains internal null bytes.
    #[inline]
    pub fn x_label(mut self, label: &str) -> Self {
        self.x_label = CString::new(label)
            .unwrap_or_else(|_| panic!("String contains internal null bytes: {}", label));
        self
    }

    /// Set the y label of the plot
    ///
    /// # Panics
    /// Will panic if the label string contains internal null bytes.
    #[inline]
    pub fn y_label(mut self, label: &str) -> Self {
        self.y_label = CString::new(label)
            .unwrap_or_else(|_| panic!("String contains internal null bytes: {}", label));
        self
    }

    /// Set the x limits of the plot.
    ///
    /// Note: This conflicts with `linked_x_limits`, whichever is called last on plot construction
    /// takes effect.
    #[inline]
    pub fn x_limits<L: Into<ImPlotRange>>(
        mut self,
        limits: L,
        condition: Condition,
        axis: Axis
    ) -> Self {
        if let Some(axis_index) = get_x_axis_index(axis) {
            self.x_limits[axis_index] = Some(AxisLimitSpecification::Single(limits.into(), condition));
        }
        self
    }

    /// Convenience function to directly set the X limits for the first X axis. To programmatically
    /// (or on demand) decide which axis to set limits for, use [`Plot::x_limits`]
    #[inline]
    pub fn x1_limits<L: Into<ImPlotRange>>(self, limits: L, condition: Condition) -> Self {
        self.x_limits(limits, condition, Axis::X1)
    }

    /// Convenience function to directly set the X limits for the secondXY axis. To
    /// programmatically (or on demand) decide which axis to set limits for, use [`Plot::x_limits`]
    #[inline]
    pub fn x2_limits<L: Into<ImPlotRange>>(self, limits: L, condition: Condition) -> Self {
        self.x_limits(limits, condition, Axis::X2)
    }

    /// Convenience function to directly set the X limits for the third X axis. To programmatically
    /// (or on demand) decide which axis to set limits for, use [`Plot::x_limits`]
    #[inline]
    pub fn x3_limits<L: Into<ImPlotRange>>(self, limits: L, condition: Condition) -> Self {
        self.x_limits(limits, condition, Axis::X3)
    }

    /// Set linked x limits for this plot. Pass clones of the same `Rc` into other plots
    /// to link their limits with the same values. Call multiple times with different
    /// `axis` values to set for multiple axes, or use the convenience methods such as
    /// [`Plot::x1_limits`]. This function requires that the axis value refers to some X axis
    /// otherwise this will be a no-op
    ///
    /// Note: This conflicts with `x_limits`, whichever is called last on plot construction takes
    /// effect.
    #[inline]
    pub fn linked_x_limits(
        mut self,
        limits: Rc<RefCell<ImPlotRange>>,
        axis: Axis
    ) -> Self {
        if let Some(axis_index) = get_x_axis_index(axis) {
            self.x_limits[axis_index] = Some(AxisLimitSpecification::Linked(limits));
        }
        self
    }

    /// Convenience function to directly set linked X limits for the first X axis. To
    /// programmatically (or on demand) decide which axis to set limits for, use
    /// [`Plot::linked_x_limits`].
    #[inline]
    pub fn linked_x1_limits(self, limits: Rc<RefCell<ImPlotRange>>) -> Self {
        self.linked_x_limits(limits, Axis::X1)
    }

    /// Convenience function to directly set linked X limits for the second X axis. To
    /// programmatically (or on demand) decide which axis to set limits for, use
    /// [`Plot::linked_x_limits`].
    #[inline]
    pub fn linked_x2_limits(self, limits: Rc<RefCell<ImPlotRange>>) -> Self {
        self.linked_x_limits(limits, Axis::X2)
    }

    /// Convenience function to directly set linked X limits for the third X axis. To
    /// programmatically (or on demand) decide which axis to set limits for, use
    /// [`Plot::linked_x_limits`].
    #[inline]
    pub fn linked_x3_limits(self, limits: Rc<RefCell<ImPlotRange>>) -> Self {
        self.linked_x_limits(limits, Axis::X3)
    }

    /// Set the Y limits of the plot for the given Y axis. Call multiple times with different
    /// `y_axis_choice` values to set for multiple axes, or use the convenience methods such as
    /// [`Plot::y1_limits`].
    ///
    /// Note: This conflicts with `linked_y_limits`, whichever is called last on plot construction
    /// takes effect for a given axis.
    #[inline]
    pub fn y_limits<L: Into<ImPlotRange>>(
        mut self,
        limits: L,
        condition: Condition,
        axis: Axis
    ) -> Self {
        if let Some(axis_index) = get_y_axis_index(axis) {
            self.y_limits[axis_index] = Some(AxisLimitSpecification::Single(limits.into(), condition));
        }
        self
    }

    /// Convenience function to directly set the Y limits for the first Y axis. To programmatically
    /// (or on demand) decide which axis to set limits for, use [`Plot::y_limits`]
    #[inline]
    pub fn y1_limits<L: Into<ImPlotRange>>(self, limits: L, condition: Condition) -> Self {
        self.y_limits(limits, condition, Axis::Y1)
    }

    /// Convenience function to directly set the Y limits for the second Y axis. To
    /// programmatically (or on demand) decide which axis to set limits for, use [`Plot::y_limits`]
    #[inline]
    pub fn y2_limits<L: Into<ImPlotRange>>(self, limits: L, condition: Condition) -> Self {
        self.y_limits(limits, condition, Axis::Y2)
    }

    /// Convenience function to directly set the Y limits for the third Y axis. To programmatically
    /// (or on demand) decide which axis to set limits for, use [`Plot::y_limits`]
    #[inline]
    pub fn y3_limits<L: Into<ImPlotRange>>(self, limits: L, condition: Condition) -> Self {
        self.y_limits(limits, condition, Axis::Y3)
    }

    /// Set linked Y limits of the plot for the given Y axis. Pass clones of the same `Rc` into
    /// other plots to link their limits with the same values. Call multiple times with different
    /// `axis` values to set for multiple axes, or use the convenience methods such as
    /// [`Plot::y1_limits`]. This function requires that the axis value refers to some Y axis
    /// otherwise this will be a no-op
    ///
    /// Note: This conflicts with `y_limits`, whichever is called last on plot construction takes
    /// effect for a given axis.
    #[inline]
    pub fn linked_y_limits(
        mut self,
        limits: Rc<RefCell<ImPlotRange>>,
        axis: Axis,
    ) -> Self {
        if let Some(axis_index) = get_y_axis_index(axis) {
            let axis_index = axis as usize - Axis::Y1 as usize;
            self.y_limits[axis_index] = Some(AxisLimitSpecification::Linked(limits));
        }
        self
    }

    /// Convenience function to directly set linked Y limits for the first Y axis. To
    /// programmatically (or on demand) decide which axis to set limits for, use
    /// [`Plot::linked_y_limits`].
    #[inline]
    pub fn linked_y1_limits(self, limits: Rc<RefCell<ImPlotRange>>) -> Self {
        self.linked_y_limits(limits, Axis::Y1)
    }

    /// Convenience function to directly set linked Y limits for the second Y axis. To
    /// programmatically (or on demand) decide which axis to set limits for, use
    /// [`Plot::linked_y_limits`].
    #[inline]
    pub fn linked_y2_limits(self, limits: Rc<RefCell<ImPlotRange>>) -> Self {
        self.linked_y_limits(limits, Axis::Y2)
    }

    /// Convenience function to directly set linked Y limits for the third Y axis. To
    /// programmatically (or on demand) decide which axis to set limits for, use
    /// [`Plot::linked_y_limits`].
    #[inline]
    pub fn linked_y3_limits(self, limits: Rc<RefCell<ImPlotRange>>) -> Self {
        self.linked_y_limits(limits, Axis::Y3)
    }

    /// Set X ticks without labels for the plot. The vector contains one label each in
    /// the form of a tuple `(label_position, label_string)`. The `show_default` setting
    /// determines whether the default ticks are also shown.
    #[inline]
    pub fn x_ticks(
        mut self,
        axis: Axis,
        ticks: &[f64],
        show_default: bool
    ) -> Self {
        if let Some(axis_index) = get_x_axis_index(axis) {
            self.x_tick_positions[axis_index] = Some(ticks.into());
            self.show_x_default_ticks[axis_index] = show_default;
        }
        self
    }

    /// Set X ticks without labels for the plot. The vector contains one label each in
    /// the form of a tuple `(label_position, label_string)`. The `show_default` setting
    /// determines whether the default ticks are also shown.
    #[inline]
    pub fn y_ticks(
        mut self,
        axis: Axis,
        ticks: &[f64],
        show_default: bool,
    ) -> Self {
        if let Some(axis_index) = get_y_axis_index(axis) {
            self.y_tick_positions[axis_index] = Some(ticks.into());
            self.show_y_default_ticks[axis_index] = show_default;
        }
        self
    }

    /// Set X ticks with labels for the plot. The vector contains one position and label
    /// each in the form of a tuple `(label_position, label_string)`. The `show_default`
    /// setting determines whether the default ticks are also shown.
    ///
    /// # Panics
    /// Will panic if any of the tick label strings contain internal null bytes.
    #[inline]
    pub fn x_ticks_with_labels(
        mut self,
        axis: Axis,
        tick_labels: &[(f64, String)],
        show_default: bool,
    ) -> Self {
        if let Some(axis_index) = get_x_axis_index(axis) {
            self.x_tick_positions[axis_index] = Some(tick_labels.iter().map(|x| x.0).collect());
            self.x_tick_labels[axis_index] = Some(
                tick_labels
                    .iter()
                    .map(|x| {
                        CString::new(x.1.as_str())
                            .unwrap_or_else(|_| panic!("String contains internal null bytes: {}", x.1))
                    })
                    .collect(),
            );
            self.show_x_default_ticks[axis_index] = show_default;
        }
        self
    }

    /// Set Y ticks with labels for the plot. The vector contains one position and label
    /// each in the form of a tuple `(label_position, label_string)`. The `show_default`
    /// setting determines whether the default ticks are also shown.
    ///
    /// # Panics
    /// Will panic if any of the tick label strings contain internal null bytes.
    #[inline]
    pub fn y_ticks_with_labels(
        mut self,
        axis: Axis,
        tick_labels: &[(f64, String)],
        show_default: bool,
    ) -> Self {
        if let Some(axis_index) = get_y_axis_index(axis) {
            self.y_tick_positions[axis_index] = Some(tick_labels.iter().map(|x| x.0).collect());
            self.y_tick_labels[axis_index] = Some(
                tick_labels
                    .iter()
                    .map(|x| {
                        CString::new(x.1.as_str())
                            .unwrap_or_else(|_| panic!("String contains internal null bytes: {}", x.1))
                    })
                    .collect(),
            );
            self.show_y_default_ticks[axis_index] = show_default;
        }
        self
    }

    /// Set the plot flags, see the help for `PlotFlags` for what the available flags are
    #[inline]
    pub fn with_plot_flags(mut self, flags: &PlotFlags) -> Self {
        self.plot_flags = *flags;
        self
    }

    /// Set the axis flags for the X axis in this plot
    #[inline]
    pub fn with_x_axis_flags(mut self, axis: Axis, flags: &AxisFlags) -> Self {
        if let Some(axis_index) = get_x_axis_index(axis) {
            self.x_flags[axis_index] = *flags;
        }
        self
    }

    /// Set the axis flags for the selected Y axis in this plot
    #[inline]
    pub fn with_y_axis_flags(mut self, axis: Axis, flags: &AxisFlags) -> Self {
        if let Some(axis_index) = get_y_axis_index(axis) {
            self.y_flags[axis_index] = *flags;
        }
        self
    }

    /// Set the legend location and configuration flags
    #[rustversion::attr(since(1.48), doc(alias = "SetLegendLocation"))]
    #[inline]
    pub fn with_legend_location(
        mut self,
        location: &PlotLocation,
        flags: LegendFlags
    ) -> Self {
        self.legend_configuration = Some((*location, flags));
        self
    }

    /// Internal helper function to set axis limits in case they are specified.
    fn maybe_set_axis_limits(&self) {
        // Limit-setting can either happen via direct limits or through linked limits. The version
        // of implot we link to here has different APIs for the two (separate per-axis calls for
        // direct, and one call for everything together for linked), hence the code here is a bit
        // clunky and takes the two approaches separately instead of a unified "match".

        // --- Direct limit-setting ---
        self.x_limits
            .iter()
            .enumerate()
            .for_each(|(k, limit_spec)| {
                if let Some(AxisLimitSpecification::Single(limits, condition)) = limit_spec {
                    unsafe {
                        sys::ImPlot_SetNextAxisLimits(
                            get_x_axis_from_index(k).unwrap() as i32,
                            limits.Min,
                            limits.Max,
                            *condition as sys::ImGuiCond,
                        );
                    }
                }
            });

        self.y_limits
            .iter()
            .enumerate()
            .for_each(|(k, limit_spec)| {
                if let Some(AxisLimitSpecification::Single(limits, condition)) = limit_spec {
                    unsafe {
                        sys::ImPlot_SetNextAxisLimits(
                            get_y_axis_from_index(k).unwrap() as i32,
                            limits.Min,
                            limits.Max,
                            *condition as sys::ImGuiCond,
                        );
                    }
                }
            });

        // --- Linked limit-setting ---
        let x_limit_pointers: Vec<(*mut f64, *mut f64)> = self
            .x_limits
            .iter()
            .map(|limit_spec| {
                if let Some(AxisLimitSpecification::Linked(value)) = limit_spec {
                    let mut borrowed = value.borrow_mut();
                    (
                        &mut (*borrowed).Min as *mut _,
                        &mut (*borrowed).Max as *mut _,
                    )
                } else {
                    (std::ptr::null_mut(), std::ptr::null_mut())
                }
            })
            .collect();

        let y_limit_pointers: Vec<(*mut f64, *mut f64)> = self
            .y_limits
            .iter()
            .map(|limit_spec| {
                if let Some(AxisLimitSpecification::Linked(value)) = limit_spec {
                    let mut borrowed = value.borrow_mut();
                    (
                        &mut (*borrowed).Min as *mut _,
                        &mut (*borrowed).Max as *mut _,
                    )
                } else {
                    (std::ptr::null_mut(), std::ptr::null_mut())
                }
            })
            .collect();

        unsafe {
            // Calling this unconditionally here as calling it with all NULL pointers should not
            // affect anything. In terms of unsafety, the pointers should be OK as long as any plot
            // struct that has an Rc to the same data is alive.
            for (i, p) in x_limit_pointers.iter().enumerate() {
                sys::ImPlot_SetNextAxisLinks(get_x_axis_from_index(i).unwrap() as i32, p.0, p.1 );
            }
            for (i, p) in y_limit_pointers.iter().enumerate() {
                sys::ImPlot_SetNextAxisLinks(get_y_axis_from_index(i).unwrap() as i32, p.0, p.1 );
            }
        }
    }

    /// Internal helper function to set tick labels in case they are specified. This does the
    /// preparation work that is the same for both the X and Y axis plots, then calls the
    /// "set next plot ticks" wrapper functions for both X and Y.
    fn maybe_set_tick_labels(&self) {

        // Show x ticks if they are available
        self.x_tick_positions
            .iter()
            .zip(self.x_tick_labels.iter())
            .zip(self.show_x_default_ticks.iter())
            .enumerate()
            .for_each(|(k, ((positions, labels), show_defaults))| {
                if positions.is_some() && !positions.as_ref().unwrap().is_empty() {
                    // The vector of pointers we create has to have a longer lifetime
                    let mut pointer_vec;
                    let labels_pointer = if let Some(labels_value) = &labels {
                        pointer_vec = labels_value
                            .iter()
                            .map(|x| x.as_ptr() as *const c_char)
                            .collect::<Vec<*const c_char>>();
                        pointer_vec.as_mut_ptr()
                    } else {
                        std::ptr::null_mut()
                    };

                    unsafe {
                        sys::ImPlot_SetupAxisTicks_doublePtr(
                            get_x_axis_from_index(k).unwrap() as i32,
                            positions.as_ref().unwrap().as_ptr(),
                            positions.as_ref().unwrap().len() as i32,
                            labels_pointer,
                            *show_defaults,
                        )
                    }
                }
            });

        self.y_tick_positions
            .iter()
            .zip(self.y_tick_labels.iter())
            .zip(self.show_y_default_ticks.iter())
            .enumerate()
            .for_each(|(k, ((positions, labels), show_defaults))| {
                if positions.is_some() && !positions.as_ref().unwrap().is_empty() {
                    // The vector of pointers we create has to have a longer lifetime
                    let mut pointer_vec;
                    let labels_pointer = if let Some(labels_value) = &labels {
                        pointer_vec = labels_value
                            .iter()
                            .map(|x| x.as_ptr() as *const c_char)
                            .collect::<Vec<*const c_char>>();
                        pointer_vec.as_mut_ptr()
                    } else {
                        std::ptr::null_mut()
                    };

                    unsafe {
                        sys::ImPlot_SetupAxisTicks_doublePtr(
                            get_y_axis_from_index(k).unwrap() as i32,
                            positions.as_ref().unwrap().as_ptr(),
                            positions.as_ref().unwrap().len() as i32,
                            labels_pointer,
                            *show_defaults,
                        )
                    }
                }
            });
    }

    /// Attempt to show the plot. If this returns a token, the plot will actually
    /// be drawn. In this case, use the drawing functionality to draw things on the
    /// plot, and then call `end()` on the token when done with the plot.
    /// If none was returned, that means the plot is not rendered.
    ///
    /// For a convenient implementation of all this, use [`build()`](struct.Plot.html#method.build)
    /// instead.
    #[rustversion::attr(since(1.48), doc(alias = "BeginPlot"))]
    pub fn begin(&self, plot_ui: &PlotUi) -> Option<PlotToken> {
        self.maybe_set_axis_limits();
        self.maybe_set_tick_labels();
        let should_render = unsafe {
            let size_vec: ImVec2 = ImVec2 { x: self.size[0], y: self.size[1], };
            sys::ImPlot_BeginPlot( self.title.as_ptr(),  size_vec,  self.plot_flags.bits() as i32 )
        };

        if should_render {
            unsafe {
                sys::ImPlot_SetupAxis(crate::Axis::X1 as i32, self.x_label.as_ptr(), self.x_flags[0].bits() as i32);
                sys::ImPlot_SetupAxis(crate::Axis::Y1 as i32, self.y_label.as_ptr(), self.y_flags[0].bits() as i32);
                // sys::ImPlot_SetupAxis(crate::Axis::Y2 as i32, self.y_label.as_ptr(), self.y_flags[1].bits() as i32);
                // sys::ImPlot_SetupAxis(crate::Axis::Y3 as i32, self.y_label.as_ptr(), self.y_flags[2].bits() as i32);
            }
            // Configure legend location, if one was set. This has to be called between begin() and
            // end(), but since only the last call to it actually affects the outcome, I'm adding
            // it here instead of as a freestanding function. If this is too restrictive (for
            // example, if you want to set the location based on code running _during_ the plotting
            // for some reason), file an issue and we'll move it.
            if let Some(legend_config) = &self.legend_configuration {
                // We introduce variables with typechecks here to safeguard against accidental
                // changes in order in the config tuple
                let location: PlotLocation = legend_config.0;
                let flags: LegendFlags = legend_config.1;
                unsafe { sys::ImPlot_SetupLegend(location as i32, flags.bits() as i32) }
            }

            Some(PlotToken {
                context: plot_ui.context,
                plot_title: self.title.clone(),
            })
        } else {
            // In contrast with imgui windows, end() does not have to be
            // called if we don't render. This is more like an imgui popup modal.
            None
        }
    }

    /// Creates a window and runs a closure to construct the contents. This internally
    /// calls `begin` and `end`.
    ///
    /// Note: the closure is not called if ImPlot::BeginPlot() returned
    /// false - TODO(4bb4) figure out if this is if things are not rendered
    #[rustversion::attr(since(1.48), doc(alias = "BeginPlot"))]
    #[rustversion::attr(since(1.48), doc(alias = "EndPlot"))]
    pub fn build<F: FnOnce()>(self, plot_ui: &PlotUi, f: F) {
        if let Some(token) = self.begin(plot_ui) {
            f();
            token.end()
        }
    }
}

/// Tracks a plot that must be ended by calling `.end()`
pub struct PlotToken {
    context: *const Context,
    /// For better error messages
    plot_title: CString,
}

impl PlotToken {
    /// End a previously begin()'ed plot.
    #[rustversion::attr(since(1.48), doc(alias = "EndPlot"))]
    pub fn end(mut self) {
        self.context = std::ptr::null();
        unsafe { sys::ImPlot_EndPlot() };
    }
}

impl Drop for PlotToken {
    fn drop(&mut self) {
        if !self.context.is_null() && !std::thread::panicking() {
            panic!(
                "Warning: A PlotToken for plot \"{:?}\" was not called end() on",
                self.plot_title
            );
        }
    }
}
