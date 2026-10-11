//! The components. One file each, named after the registry key, so the file
//! name, the registry key and the module name agree. `theme` is shared: the CLI
//! writes it once however many components are installed, and a component reads
//! it as `super::theme`, which keeps the installed directory movable.

pub mod theme;

pub mod button;
