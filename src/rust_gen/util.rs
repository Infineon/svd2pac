use convert_case::{Case, Casing};
use lazy_regex::regex;
use log::warn;
pub trait ToSanitizedSymbol {
    fn to_sanitized_mod_ident(&self) -> String;
    fn to_sanitized_struct_ident(&self) -> String;
    fn to_sanitized_func_ident(&self) -> String;
    fn to_sanitized_enum_ident(&self) -> String;
    fn to_sanitized_const_ident(&self) -> String;
    fn to_sanitized_ident(&self) -> String;
    fn to_internal_ident(&self) -> String;
    fn remove_invalid_char(&self) -> String;
}

impl ToSanitizedSymbol for str {
    fn to_sanitized_mod_ident(&self) -> String {
        self.to_internal_ident()
            .remove_invalid_char()
            .to_lowercase()
            .to_sanitized_ident()
    }
    fn to_sanitized_struct_ident(&self) -> String {
        self.to_internal_ident()
            .remove_invalid_char()
            .to_case(Case::Pascal)
            .to_sanitized_ident()
    }
    fn to_sanitized_func_ident(&self) -> String {
        self.to_internal_ident()
            .remove_invalid_char()
            .to_lowercase()
            .to_sanitized_ident()
    }
    fn to_sanitized_enum_ident(&self) -> String {
        self.to_internal_ident()
            .remove_invalid_char()
            .to_case(Case::Pascal)
            .to_sanitized_ident()
    }
    fn to_sanitized_const_ident(&self) -> String {
        self.to_internal_ident()
            .remove_invalid_char()
            .to_case(Case::ScreamingSnake)
            .to_sanitized_ident()
    }
    fn to_sanitized_ident(&self) -> String {
        // Use RAW_IDENTIFIER (i.e. prepend 'r#') if string is a Rust keyword.
        // Prepend a _ if string does not start with XID_Start character.
        let result = match syn::parse_str::<syn::Ident>(self) {
            Ok(_) => self.to_owned(),
            Err(_) => match syn::parse_str::<syn::Ident>(&("r#".to_owned() + self)) {
                Ok(_) => "r#".to_owned() + self,
                Err(_) => "_".to_owned() + self,
            },
        };
        if result != self {
            warn!("Identifier {self} sanitized to {result}");
        }
        result
    }
    fn to_internal_ident(&self) -> String {
        // FIXME: We assume that nobody creates different array of registers that differs
        // only for index position. Is this a good assumption ?
        self.replace("[%s]", "").replace("%s", "")
    }
    fn remove_invalid_char(&self) -> String {
        // Remove all char that are not valid for Rust identifier
        let reg_ex = regex!("[^a-zA-Z0-9_]");
        reg_ex.replace_all(self, "_").into_owned()
    }
}
