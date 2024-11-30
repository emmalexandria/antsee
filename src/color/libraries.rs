/*!

[CssColors](libraries::CssColors) and [XtermColors](libraries::XtermColors) are re-exported from private modules which contain macros to generate color enums.
These enums implement [ColorLibrary](libraries::ColorLibrary).

*/

mod css;
mod xterm;

#[doc(inline)]
pub use css::CssColors;
#[doc(inline)]
pub use xterm::XtermColors;

///Trait defining common functions for macro based colour libraries ([xterm], [css])
pub trait ColorLibrary
where
    Self: Sized,
{
    ///The function style wrapper which identifies a value as being from this color library
    const WRAPPER: &str;

    ///Wrap a string in the style wrapper
    fn wrap_name(s: &str) -> String;
    ///Extract a string from the style wrapper
    fn unwrap_name(s: &str) -> &str;

    ///Get a color by name
    fn get_name(s: &str) -> Option<Self>;

    ///Get the name of a color
    fn color_name(&self) -> &'static str;

    ///Get the RGB value of a colour
    fn rgb(&self) -> [u8; 3];
}
