/// `jport` - short for JVM import - is a Rust crate for importing and analising JVM bytecode. It can import both javas `class` and `jar` files. It is also very fast, capable of parsing tens of thousands of classes in under a second. It aims to provide a safe API which exposes the data in a manner closely related to how the data is stored on disk.
mod opcodes;
mod attribute;
mod import;
mod field;
type IString = Box<str>;
pub use crate::import::{JavaClassFile,JavaImportError};
pub use crate::import::{Utf8,ClassInfo};
fn import_class_file<R:std::io::Read>(mut r:R)->Result<JavaClassFile,JavaImportError>{
    crate::import::load_class(&mut r)
}
