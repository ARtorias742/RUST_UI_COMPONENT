//! Framework-agnostic primitives shared by every Ferrum UI target.
//!
//! The core crate deliberately has no renderer dependency. Web, SSR and native
//! adapters consume these values and decide how to materialise them.

pub mod a11y;
pub mod event;
pub mod focus;
pub mod portal;
pub mod roving;
pub mod ssr;
pub mod style;
pub mod tokens;

pub use a11y::{Aria, Live, Role};
pub use event::{FerrumEventExt, Key, KeyEvent};
pub use focus::{FocusManager, FocusScope};
pub use portal::{Portal, PortalHost};
pub use roving::{Orientation, RovingTabindex};
pub use ssr::{Element, Node};
pub use style::{Class, Style};
pub use tokens::{Direction, Scheme, Theme, Token, Tokens};
