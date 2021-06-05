use crate::shebang;
use crate::modeline;
use crate::data_structures::Text;

include!(concat!(env!("OUT_DIR"), "/linguist_interpreters.rs"));
include!(concat!(env!("OUT_DIR"), "/linguist_aliases.rs"));

pub fn  guess_by_interpreter(content: &Text) -> Option<&'static String> {
    if let Some(interpreter) = shebang::interpreter(&content.first_line) {
        return INTERPRETERS.get(&interpreter);
    }
    None
}

pub fn  guess_by_modeline(content: &Text) -> Option<&'static String> {
    if let Some(alias) = modeline::parse_modeline(&content.first_line) {
        return ALIASES.get(alias);
    }
    None
}