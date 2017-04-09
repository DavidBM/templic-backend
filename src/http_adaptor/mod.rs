pub mod adaptor;
pub mod endpoints;

pub use self::adaptor::HttpAdaptor;
use self::endpoints::declare_endpoints;