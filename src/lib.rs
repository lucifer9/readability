#[macro_use]
extern crate html5ever;
extern crate markup5ever;
extern crate regex;
extern crate tendril;
extern crate url;
#[macro_use]
extern crate lazy_static;
#[cfg(feature = "reqwest")]
extern crate reqwest;

pub mod dom;
pub mod error;
pub mod extractor;
pub mod markup5ever_arcdom;
pub mod scorer;
