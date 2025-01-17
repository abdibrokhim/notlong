// src/routes/mod.rs
mod shorten;
mod redirect;
mod wcreate;
mod wupdate;

pub use shorten::*;
pub use redirect::*;
pub use wcreate::*;
pub use wupdate::*;
