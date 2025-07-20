//! # Rust bindings to ImPlot
//!
//! This crate contains idiomatic bindings to the C++ [implot library](https://github.com/epezent/implot),
//! which use the bindings exposed by the `implot-sys` crate. An attempt is made to keep
//! documentation here somewhat self-contained, but when in doubt, the documentation of implot
//! itself (in particular also the demo code [here](https://github.com/epezent/implot/blob/master/implot_demo.cpp))
//! should help as well.
//!
//! For usage examples, see the `implot-examples` crate - it contains standalone runnable examples
//! that showcase the API and features of this crate. The [Github readme](https://github.com/4bb4/implot-rs)
//! lists the features that are already implemented as idiomatic bindings. For everything else, if
//! you'd really like a particular feature, file an issue and it'll be given priority for wrapping,
//! or directly contribute a PR, or use the low-level bindings directly for the time being.
//!
//! If you've seen a construct or feature in C++ implot and can't find it here, try searching for
//! the C++ name - some doc aliases are defined to increase the chances of that working. If this
//! does not yield any results, you can also try cloning the source and doing a full-text search to
//! see if the feature is used somewhere internally the code.
use implot_sys as sys;

// TODO(4bb4) facade-wrap these?
pub use self::{context::*, plot::*, plot_elements::*};
use std::{
    mem::MaybeUninit,
    os::raw::c_char
};
pub use sys::{ImPlotRect, ImPlotPoint, ImPlotRange, ImVec2, ImVec4};

mod context;
mod plot;
mod plot_elements;

// The bindings for some reason don't contain this - it has to match the IMPLOT_AUTO from
// the original C++ header for things to work properly.
const IMPLOT_AUTO: i32 = -1;

// Number of X axes, this is used in a bunch of places for storing things like settings.
const NUMBER_OF_X_AXES: usize = 3;

// Number of Y axes, this is used in a bunch of places for storing things like settings.
const NUMBER_OF_Y_AXES: usize = 3;

/// Choice of axis (X axis, followed by Y axis).
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u32)]
pub enum Axis {
    X1 = sys::ImAxis__ImAxis_X1 as u32,
    X2 = sys::ImAxis__ImAxis_X2 as u32,
    X3 = sys::ImAxis__ImAxis_X3 as u32,
    Y1 = sys::ImAxis__ImAxis_Y1 as u32,
    Y2 = sys::ImAxis__ImAxis_Y2 as u32,
    Y3 = sys::ImAxis__ImAxis_Y3 as u32,
}

/// Turn an Option<Axis> into an i32. Picks IMPLOT_AUTO for None.
#[rustversion::attr(since(1.48), doc(alias = "IMPLOT_AUTO"))]
fn axis_option_to_i32(choice: Option<Axis>) -> i32 {
    choice.map_or(IMPLOT_AUTO, |v| v as i32)
}

fn get_x_axis(axis: Axis) -> Option<Axis> {
    match axis {
        a if a < Axis::Y1 => Some(axis),
        _ => None
    }
}

fn get_x_axis_index(axis: Axis) -> Option<usize> {
    match axis {
        a if a < Axis::Y1 => Some(axis as usize),
        _ => None
    }
}

fn get_y_axis(axis: Axis) -> Option<Axis> {
    match axis {
        a if a > Axis::X3 => Some(axis),
        _ => None
    }
}

fn get_y_axis_index(axis: Axis) -> Option<usize> {
    match axis {
        a if a > Axis::X3 => Some(axis as usize - NUMBER_OF_X_AXES),
        _ => None
    }
}

fn get_x_axis_from_index(index: usize) -> Option<Axis> {
    match index {
        v if v < NUMBER_OF_X_AXES => Some(unsafe { std::mem::transmute(index as u32) }),
        _ => None
    }
}

fn get_y_axis_from_index(index: usize) -> Option<Axis> {
    match index {
        v if v < NUMBER_OF_Y_AXES => Some(unsafe { std::mem::transmute((index + NUMBER_OF_X_AXES) as u32) }),
        _ => None
    }
}

/// A temporary reference for building plots. This does not really do anything on its own at
/// this point, but it is used to enforce that a context is created and active for other features,
/// such as creating plots.
pub struct PlotUi<'ui> {
    context: &'ui Context,
}

// --- Markers, color maps, style variables, legend location ----------------------------------
/// Markers, documentation copied from implot.h for convenience.
#[rustversion::attr(since(1.48), doc(alias = "ImPlotMarker"))]
#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum Marker {
    /// no marker
    None = sys::ImPlotMarker__ImPlotMarker_None,
    /// a circle marker will be rendered at each point
    Circle = sys::ImPlotMarker__ImPlotMarker_Circle,
    /// a square maker will be rendered at each point
    Square = sys::ImPlotMarker__ImPlotMarker_Square,
    /// a diamond marker will be rendered at each point
    Diamond = sys::ImPlotMarker__ImPlotMarker_Diamond,
    /// an upward-pointing triangle marker will up rendered at each point
    Up = sys::ImPlotMarker__ImPlotMarker_Up,
    /// an downward-pointing triangle marker will up rendered at each point
    Down = sys::ImPlotMarker__ImPlotMarker_Down,
    /// an leftward-pointing triangle marker will up rendered at each point
    Left = sys::ImPlotMarker__ImPlotMarker_Left,
    /// an rightward-pointing triangle marker will up rendered at each point
    Right = sys::ImPlotMarker__ImPlotMarker_Right,
    /// a cross marker will be rendered at each point (not filled)
    Cross = sys::ImPlotMarker__ImPlotMarker_Cross,
    /// a plus marker will be rendered at each point (not filled)
    Plus = sys::ImPlotMarker__ImPlotMarker_Plus,
    /// a asterisk marker will be rendered at each point (not filled)
    Asterisk = sys::ImPlotMarker__ImPlotMarker_Asterisk,
}

/// Colorable plot elements. These are called "ImPlotCol" in ImPlot itself, but I found that
/// name somewhat confusing because we are not referring to colors, but _which_ thing can
/// be colored - hence I added the "Element".
#[rustversion::attr(since(1.48), doc(alias = "ImPlotCol"))]
#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum PlotColorElement {
    /// Plot line/outline color (defaults to next unused color in current colormap)
    Line = sys::ImPlotCol__ImPlotCol_Line as u32,
    /// Plot fill color for bars (defaults to the current line color)
    Fill = sys::ImPlotCol__ImPlotCol_Fill as u32,
    /// Marker outline color (defaults to the current line color)
    MarkerOutline = sys::ImPlotCol__ImPlotCol_MarkerOutline as u32,
    /// Marker fill color (defaults to the current line color)
    MarkerFill = sys::ImPlotCol__ImPlotCol_MarkerFill as u32,
    /// Error bar color (defaults to text color)
    ErrorBar = sys::ImPlotCol__ImPlotCol_ErrorBar as u32,
    /// Plot frame background color (defaults to FRAME_BG)
    FrameBg = sys::ImPlotCol__ImPlotCol_FrameBg as u32,
    /// Plot area background color (defaults to WINDOW_BG)
    PlotBg = sys::ImPlotCol__ImPlotCol_PlotBg as u32,
    /// Plot area border color (defaults to text color)
    PlotBorder = sys::ImPlotCol__ImPlotCol_PlotBorder as u32,
    /// Legend background color (defaults to ImGuiCol_PopupBg)
    LegendBackground = sys::ImPlotCol__ImPlotCol_LegendBg as u32,
    /// Legend border color (defaults to ImPlotCol_PlotBorder)
    LegendBorder = sys::ImPlotCol__ImPlotCol_LegendBorder as u32,
    /// Legend text color (defaults to ImPlotCol_InlayText)
    LegendText = sys::ImPlotCol__ImPlotCol_LegendText as u32,
    /// Plot title text color (defaults to ImGuiCol_Text)
    TitleText = sys::ImPlotCol__ImPlotCol_TitleText as u32,
    /// Color of text appearing inside of plots (defaults to ImGuiCol_Text)
    InlayText = sys::ImPlotCol__ImPlotCol_InlayText as u32,
    /// axis label and tick lables color (defaults to ImGuiCol_Text)
    AxisText = sys::ImPlotCol__ImPlotCol_AxisText as u32,
    /// X-axis grid color (defaults to 25% ImPlotCol_XAxis)
    AxisGrid = sys::ImPlotCol__ImPlotCol_AxisGrid as u32,
    /// axis tick color (defaults to AxisGrid)
    AxisTick = sys::ImPlotCol__ImPlotCol_AxisTick as u32, 
    /// background color of axis hover region (defaults to transparent)
    AxisBg = sys::ImPlotCol__ImPlotCol_AxisBg as u32,
    /// axis hover color (defaults to ImGuiCol_ButtonHovered)
    AxisBgHovered = sys::ImPlotCol__ImPlotCol_AxisBgHovered as u32,
    /// axis active color (defaults to ImGuiCol_ButtonActive)
    AxisBgActive = sys::ImPlotCol__ImPlotCol_AxisBgActive as u32,
    /// Box-selection color (defaults to yellow)
    Selection = sys::ImPlotCol__ImPlotCol_Selection as u32,
    /// crosshairs color (defaults to ImPlotCol_PlotBorder)
    Crosshairs = sys::ImPlotCol__ImPlotCol_Crosshairs as u32,
}

/// Colormap choice. Documentation copied from implot.h for convenience.
#[rustversion::attr(since(1.48), doc(alias = "ImPlotColormap"))]
#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum Colormap {
    /// a.k.a. seaborn deep (n=10)
    Deep = sys::ImPlotColormap__ImPlotColormap_Deep as u32,
    /// a.k.a. matplotlib "Set1" (n=9)
    Dark = sys::ImPlotColormap__ImPlotColormap_Dark as u32,
    /// a.k.a. matplotlib "Pastel1" (n=9)
    Pastel = sys::ImPlotColormap__ImPlotColormap_Pastel as u32,
    /// a.k.a. matplotlib "Paired" (n=12)
    Paired = sys::ImPlotColormap__ImPlotColormap_Paired as u32,
    /// a.k.a. matplotlib "viridis" (n=11)
    Viridis = sys::ImPlotColormap__ImPlotColormap_Viridis as u32,
    /// a.k.a. matplotlib "plasma" (n=11)
    Plasma = sys::ImPlotColormap__ImPlotColormap_Plasma as u32,
    /// a.k.a. matplotlib/MATLAB "hot" (n=11)
    Hot = sys::ImPlotColormap__ImPlotColormap_Hot as u32,
    /// a.k.a. matplotlib/MATLAB "cool" (n=11)
    Cool = sys::ImPlotColormap__ImPlotColormap_Cool as u32,
    /// a.k.a. matplotlib/MATLAB "pink" (n=11)
    Pink = sys::ImPlotColormap__ImPlotColormap_Pink as u32,
    /// a.k.a. MATLAB "jet" (n=11)
    Jet = sys::ImPlotColormap__ImPlotColormap_Jet as u32,
    /// a.k.a. matplotlib "twilight" (n=11)
    Twilight = sys::ImPlotColormap__ImPlotColormap_Twilight as u32,
    /// red/blue, Color Brewer
    RdBu = sys::ImPlotColormap__ImPlotColormap_RdBu as u32,
    /// brown/blue-green, Color Brewer
    BrBG = sys::ImPlotColormap__ImPlotColormap_BrBG as u32,
    /// pink/yellow-green, Color Brewer
    PiYG = sys::ImPlotColormap__ImPlotColormap_PiYG as u32,
    /// color spectrum, Color Brewer
    Spectral = sys::ImPlotColormap__ImPlotColormap_Spectral as u32,
    /// white/black
    Greys = sys::ImPlotColormap__ImPlotColormap_Greys as u32,
}

/// Style variable choice, as in "which thing will be affected by a style setting".
#[rustversion::attr(since(1.48), doc(alias = "ImPlotStyleVar"))]
#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum StyleVar {
    /// f32, line weight in pixels
    LineWeight = sys::ImPlotStyleVar__ImPlotStyleVar_LineWeight as u32,
    /// u32,  marker specification
    Marker = sys::ImPlotStyleVar__ImPlotStyleVar_Marker as u32,
    /// f32, marker size in pixels (roughly the marker's "radius")
    MarkerSize = sys::ImPlotStyleVar__ImPlotStyleVar_MarkerSize as u32,
    /// f32, outline weight of markers in pixels
    MarkerWeight = sys::ImPlotStyleVar__ImPlotStyleVar_MarkerWeight as u32,
    /// f32, alpha modifier applied to all plot item fills
    FillAlpha = sys::ImPlotStyleVar__ImPlotStyleVar_FillAlpha as u32,
    /// f32, error bar whisker width in pixels
    ErrorBarSize = sys::ImPlotStyleVar__ImPlotStyleVar_ErrorBarSize as u32,
    /// f32, error bar whisker weight in pixels
    ErrorBarWeight = sys::ImPlotStyleVar__ImPlotStyleVar_ErrorBarWeight as u32,
    /// f32, digital channels bit height (at 1) in pixels
    DigitalBitHeight = sys::ImPlotStyleVar__ImPlotStyleVar_DigitalBitHeight as u32,
    /// f32, digital channels bit padding gap in pixels
    DigitalBitGap = sys::ImPlotStyleVar__ImPlotStyleVar_DigitalBitGap as u32,
    /// f32,  thickness of border around plot area
    PlotBorderSize = sys::ImPlotStyleVar__ImPlotStyleVar_PlotBorderSize as u32,
    /// f32,  alpha multiplier applied to minor axis grid lines
    MinorAlpha = sys::ImPlotStyleVar__ImPlotStyleVar_MinorAlpha as u32,
    /// ImVec2, major tick lengths for X and Y axes
    MajorTickLen = sys::ImPlotStyleVar__ImPlotStyleVar_MajorTickLen as u32,
    /// ImVec2, minor tick lengths for X and Y axes
    MinorTickLen = sys::ImPlotStyleVar__ImPlotStyleVar_MinorTickLen as u32,
    /// ImVec2, line thickness of major ticks
    MajorTickSize = sys::ImPlotStyleVar__ImPlotStyleVar_MajorTickSize as u32,
    /// ImVec2, line thickness of minor ticks
    MinorTickSize = sys::ImPlotStyleVar__ImPlotStyleVar_MinorTickSize as u32,
    /// ImVec2, line thickness of major grid lines
    MajorGridSize = sys::ImPlotStyleVar__ImPlotStyleVar_MajorGridSize as u32,
    /// ImVec2, line thickness of minor grid lines
    MinorGridSize = sys::ImPlotStyleVar__ImPlotStyleVar_MinorGridSize as u32,
    /// ImVec2, padding between widget frame and plot area and/or labels
    PlotPadding = sys::ImPlotStyleVar__ImPlotStyleVar_PlotPadding as u32,
    /// ImVec2, padding between axes labels, tick labels, and plot edge
    LabelPadding = sys::ImPlotStyleVar__ImPlotStyleVar_LabelPadding as u32,
    /// ImVec2, legend padding from top-left of plot
    LegendPadding = sys::ImPlotStyleVar__ImPlotStyleVar_LegendPadding as u32,
    /// ImVec2, legend inner padding from legend edges
    LegendInnerPadding = sys::ImPlotStyleVar__ImPlotStyleVar_LegendInnerPadding as u32,
    /// ImVec2, spacing between legend entries
    LegendSpacing = sys::ImPlotStyleVar__ImPlotStyleVar_LegendSpacing as u32,
    /// ImVec2, padding between plot edge and interior info text
    MousePosPadding = sys::ImPlotStyleVar__ImPlotStyleVar_MousePosPadding as u32,
    /// ImVec2, text padding around annotation labels
    AnnotationPadding = sys::ImPlotStyleVar__ImPlotStyleVar_AnnotationPadding as u32,
    /// ImVec2, additional fit padding as a percentage of the fit extents
    /// (e.g. ImVec2(0.1f,0.1f) adds 10% to the fit extents of X and Y)
    FitPadding = sys::ImPlotStyleVar__ImPlotStyleVar_FitPadding as u32,
    /// ImVec2, default size used when ImVec2(0,0) is passed to BeginPlot
    PlotDefaultSize = sys::ImPlotStyleVar__ImPlotStyleVar_PlotDefaultSize as u32,
    /// ImVec2, minimum size plot frame can be when shrunk
    PlotMinSize = sys::ImPlotStyleVar__ImPlotStyleVar_PlotMinSize as u32,
}

/// Used to position items on a plot (e.g. legends, labels, etc.)
#[rustversion::attr(since(1.48), doc(alias = "ImPlotLocation"))]
#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum PlotLocation {
    /// Center-center
    Center = sys::ImPlotLocation__ImPlotLocation_Center as u32,
    /// Top-center
    North = sys::ImPlotLocation__ImPlotLocation_North as u32,
    /// Bottom-center
    South = sys::ImPlotLocation__ImPlotLocation_South as u32,
    /// Center-left
    West = sys::ImPlotLocation__ImPlotLocation_West as u32,
    /// Center-right
    East = sys::ImPlotLocation__ImPlotLocation_East as u32,
    /// Top-left
    NorthWest = sys::ImPlotLocation__ImPlotLocation_NorthWest as u32,
    /// Top-right
    NorthEast = sys::ImPlotLocation__ImPlotLocation_NorthEast as u32,
    /// Bottom-left
    SouthWest = sys::ImPlotLocation__ImPlotLocation_SouthWest as u32,
    /// Bottom-right
    SouthEast = sys::ImPlotLocation__ImPlotLocation_SouthEast as u32,
}

/// Switch to one of the built-in preset colormaps. If samples is greater than 1, the map will be
/// linearly resampled.
#[rustversion::attr(since(1.48), doc(alias = "SetColormap"))]
pub fn set_colormap_from_preset(preset: Colormap, samples: u32) {
    unsafe {
        // "as" casts saturate as of Rust 1.45. This is safe here, and at least the enum
        // values are not expected to go outside the range of an i32 anyway, so there is no
        // risk of changed values.
        // sys::ImPlot_SetColormapPlotColormap(preset as i32, samples as i32);
    }
}

/// Set a custom colormap in the form of a vector of colors.
#[rustversion::attr(since(1.48), doc(alias = "SetColormap"))]
pub fn set_colormap_from_vec(colors: Vec<ImVec4>) {
    unsafe {
        // sys::ImPlot_SetColormapVec4Ptr(colors.as_ptr(), colors.len() as i32);
    }
}

// --- Push/pop utils -------------------------------------------------------------------------
// Currently not in a struct yet. imgui-rs has some smarts about dealing with stacks, in particular
// leak detection, which I'd like to replicate here at some point.
/// Push a style color to the stack, giving an element and the four components of the color.
/// The components should be between 0.0 (no intensity) and 1.0 (full intensity).
/// The return value is a token that gets used for removing the style color from the stack again:
/// ```no_run
/// # use implot::{push_style_color, PlotColorElement};
/// let pushed_var = push_style_color(&PlotColorElement::Line, 1.0, 1.0, 1.0, 0.2);
/// // Plot some things
/// pushed_var.pop();
/// ```
#[rustversion::attr(since(1.48), doc(alias = "PushStyleColor"))]
pub fn push_style_color(
    element: &PlotColorElement,
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
) -> StyleColorToken {
    unsafe {
        sys::ImPlot_PushStyleColor_Vec4(
            *element as sys::ImPlotCol,
            sys::ImVec4 {
                x: red,
                y: green,
                z: blue,
                w: alpha,
            },
        );
    }
    StyleColorToken { was_popped: false }
}

/// Tracks a change pushed to the style color stack
pub struct StyleColorToken {
    /// Whether this token has been popped or not.
    was_popped: bool,
}

impl StyleColorToken {
    #[rustversion::attr(since(1.48), doc(alias = "PopStyleColor"))]
    pub fn pop(mut self) {
        if self.was_popped {
            panic!("Attempted to pop a style color token twice.")
        }
        self.was_popped = true;
        unsafe {
            sys::ImPlot_PopStyleColor(1);
        }
    }
}

/// Push a f32 style variable to the stack. The returned token is used for removing
/// the variable from the stack again:
/// ```no_run
/// # use implot::{push_style_var_f32, StyleVar};
/// let pushed_var = push_style_var_f32(&StyleVar::LineWeight, 11.0);
/// // Plot some things
/// pushed_var.pop();
/// ```
#[rustversion::attr(since(1.48), doc(alias = "PushStyleVar"))]
pub fn push_style_var_f32(element: &StyleVar, value: f32) -> StyleVarToken {
    unsafe {
        sys::ImPlot_PushStyleVar_Float(*element as sys::ImPlotStyleVar, value);
    }
    StyleVarToken { was_popped: false }
}

/// Push an u32 style variable to the stack. The only i32 style variable is Marker
/// at the moment, for that, use something like
/// ```no_run
/// # use implot::{push_style_var_i32, StyleVar, Marker};
/// let markerchoice = push_style_var_i32(&StyleVar::Marker, Marker::Cross as i32);
/// // plot things
/// markerchoice.pop()
/// ```
#[rustversion::attr(since(1.48), doc(alias = "PushStyleVar"))]
pub fn push_style_var_i32(element: &StyleVar, value: i32) -> StyleVarToken {
    unsafe {
        sys::ImPlot_PushStyleVar_Int(*element as sys::ImPlotStyleVar, value);
    }
    StyleVarToken { was_popped: false }
}

/// Push an ImVec2 style variable to the stack. The returned token is used for removing
/// the variable from the stack again.
pub fn push_style_var_imvec2(element: &StyleVar, value: ImVec2) -> StyleVarToken {
    unsafe {
        sys::ImPlot_PushStyleVar_Vec2(*element as sys::ImPlotStyleVar, value);
    }
    StyleVarToken { was_popped: false }
}

/// Tracks a change pushed to the style variable stack
pub struct StyleVarToken {
    /// Whether this token has been popped or not.
    was_popped: bool,
}

impl StyleVarToken {
    /// Pop this token from the stack.
    #[rustversion::attr(since(1.48), doc(alias = "PopStyleVar"))]
    pub fn pop(mut self) {
        if self.was_popped {
            panic!("Attempted to pop a style var token twice.")
        }
        self.was_popped = true;
        unsafe {
            sys::ImPlot_PopStyleVar(1);
        }
    }
}

// --- Miscellaneous -----------------------------------------------------------------------------
/// Returns true if the plot area in the current or most recent plot is hovered.
#[rustversion::attr(since(1.48), doc(alias = "IsPlotHovered"))]
pub fn is_plot_hovered() -> bool {
    unsafe { sys::ImPlot_IsPlotHovered() }
}

/* 
/// Returns true if the current or most recent plot is queried
#[rustversion::attr(since(1.48), doc(alias = "IsPlotQueried"))]
pub fn is_plot_queried() -> bool {
    unsafe { sys::ImPlot_IsPlotQueried() }
}
*/

/// Returns the mouse position in x,y coordinates of the current or most recent plot,
/// for the specified choice of Y axis. If `None` is the Y axis choice, that means the
/// most recently selected Y axis is chosen.
#[rustversion::attr(since(1.48), doc(alias = "GetPlotMousePos"))]
pub fn get_plot_mouse_position(x_axis: Axis, y_axis: Axis) -> ImPlotPoint {
    let mut point = ImPlotPoint { X: 0.0, Y: 0.0 }; // doesn't seem to have default()
    unsafe {
        sys::ImPlot_GetPlotMousePos(&mut point as *mut ImPlotPoint, x_axis as i32, y_axis as i32);
    }
    point
}

/// Convert pixels, given as an `ImVec2`, to a position in the current plot's coordinate system.
/// Uses the specified Y axis, if any, otherwise whatever was previously chosen.
#[rustversion::attr(since(1.48), doc(alias = "PixelsToPlot"))]
pub fn pixels_to_plot_vec2(
    pixel_position: &ImVec2,
    x_axis: Axis,
    y_axis: Axis,
) -> ImPlotPoint {
    let mut point = ImPlotPoint { X: 0.0, Y: 0.0 }; // doesn't seem to have default()
    unsafe {
        sys::ImPlot_PixelsToPlot_Vec2(
            &mut point as *mut ImPlotPoint,
            *pixel_position,
            x_axis as i32,
            y_axis as i32
        );
    }
    point
}

/// Convert pixels, given as floats `x` and `y`, to a position in the current plot's coordinate
/// system. Uses the specified Y axis, if any, otherwise whatever was previously chosen.
#[rustversion::attr(since(1.48), doc(alias = "PixelsToPlot"))]
pub fn pixels_to_plot_f32(
    pixel_position_x: f32,
    pixel_position_y: f32,
    x_axis: Axis,
    y_axis: Axis,
) -> ImPlotPoint {
    let mut point = ImPlotPoint { X: 0.0, Y: 0.0 }; // doesn't seem to have default()
    unsafe {
        sys::ImPlot_PixelsToPlot_Float(
            &mut point as *mut ImPlotPoint,
            pixel_position_x,
            pixel_position_y,
            x_axis as i32,
            y_axis as i32
        );
    }
    point
}

/// Convert a position in the current plot's coordinate system to pixels. Uses the specified Y
/// axis, if any, otherwise whatever was previously chosen.
///
#[rustversion::attr(since(1.48), doc(alias = "PlotToPixels"))]
pub fn plot_to_pixels_vec2(
    plot_position: &ImPlotPoint,
    x_axis: Axis,
    y_axis: Axis,
) -> ImVec2 {
    let mut pixel_position = ImVec2 { x: 0.0, y: 0.0 }; // doesn't seem to have default()
    unsafe {
        sys::ImPlot_PlotToPixels_PlotPoInt(
            &mut pixel_position as *mut ImVec2,
            *plot_position,
            x_axis as i32,
            y_axis as i32
        );
    }
    pixel_position
}

/// Convert a position in the current plot's coordinate system to pixels. Uses the specified Y
/// axis, if any, otherwise whatever was previously chosen.
#[rustversion::attr(since(1.48), doc(alias = "PlotToPixels"))]
pub fn plot_to_pixels_f32(
    plot_position_x: f64,
    plot_position_y: f64,
    x_axis: Axis,
    y_axis: Axis,
) -> ImVec2 {
    let mut pixel_position = ImVec2 { x: 0.0, y: 0.0 }; // doesn't seem to have default()
    unsafe {
        sys::ImPlot_PlotToPixels_double(
            &mut pixel_position as *mut ImVec2,
            plot_position_x,
            plot_position_y,
            x_axis as i32,
            y_axis as i32
        );
    }
    pixel_position
}

/// Returns the current or most recent plot axis range for the specified choice of Y axis. If
/// `None` is the Y axis choice, that means the most recently selected Y axis is chosen.
#[rustversion::attr(since(1.48), doc(alias = "GetPlotLimits"))]
pub fn get_plot_limits(x_axis: Axis, y_axis: Axis) -> ImPlotRect {
    // ImPlotRect doesn't seem to have default()
    let mut limits = ImPlotRect {
        X: ImPlotRange { Min: 0.0, Max: 0.0 },
        Y: ImPlotRange { Min: 0.0, Max: 0.0 },
    };
    unsafe {
        sys::ImPlot_GetPlotLimits(&mut limits as *mut ImPlotRect, x_axis as i32, y_axis as i32);
    }
    limits
}

/// Returns the query limits of the current or most recent plot, for the specified choice of Y
/// axis. If `None` is the Y axis choice, that means the most recently selected Y axis is chosen.
#[rustversion::attr(since(1.48), doc(alias = "GetPlotQuery"))]
pub fn get_plot_query() -> ImPlotRect {
    // ImPlotRect doesn't seem to have default()
    let mut limits: MaybeUninit<ImPlotRect> = MaybeUninit::uninit();
    unsafe {
        sys::ImPlot_DragRect(
            0,
            &mut limits.assume_init_mut().X.Min, 
            &mut limits.assume_init_mut().X.Max, 
            &mut limits.assume_init_mut().X.Min, 
            &mut limits.assume_init_mut().X.Max, 
            ImVec4 { x: 0., y: 0., z: 0., w: 0. }, 
            0, 
            &mut false, 
            &mut false, 
            &mut false
        );
        limits.assume_init()
    }
}

/// Set the X or Y axis to be used for any upcoming plot elements
pub fn set_axis(axis: Axis) {
    unsafe {
        sys::ImPlot_SetAxis(axis as i32);
    }
}

/// Returns true if the axis area of the given axis choice in the current plot is hovered. If
/// `None` is the axis choice, that means the most recently selected axis is chosen.
#[rustversion::attr(since(1.48), doc(alias = "IsPlotAxisHovered"))]
pub fn is_plot_axis_hovered(axis_choice: Option<Axis>) -> bool {
    let axis_choice_i32 = axis_option_to_i32(axis_choice);
    unsafe { sys::ImPlot_IsAxisHovered(axis_choice_i32) }
}

/// Returns true if the given item in the legend of the current plot is hovered.
pub fn is_legend_entry_hovered(legend_entry: &str) -> bool {
    unsafe { sys::ImPlot_IsLegendEntryHovered(legend_entry.as_ptr() as *const c_char) }
}

// --- Demo window -------------------------------------------------------------------------------
/// Show the demo window for poking around what functionality implot has to
/// offer. Note that not all of this is necessarily implemented in implot-rs
/// already - if you find something missing you'd really like, raise an issue.
// This requires implot_demo.cpp to be in the list of sources in implot-sys.
#[rustversion::attr(since(1.48), doc(alias = "ShowDemoWindow"))]
pub fn show_demo_window(show: &mut bool) {
    unsafe {
        implot_sys::ImPlot_ShowDemoWindow(show);
    }
}
