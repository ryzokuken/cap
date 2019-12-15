// Reserved word lists for various dialects of the language

pub struct ReservedWords {
    Three: String String::from("abstract boolean byte char class double enum export extends final float goto implements import int interface long native package private protected public short static super synchronized throws transient volatile"),
    Five: String::from("class enum extends super const export import"),
    Six: String::from("enum"),
    Strict: String::from("implements interface let package private protected public static yield"),
    StrictBind: String::from("eval arguments"),
}

const ecma5AndLessKeywords: String = String::from("break case catch continue debugger default do else finally for function if return switch throw try var while with null true false instanceof typeof void delete new in this");

pub struct Keywords {
    Five = ecma5AndLessKeywords,
    FiveModule = ecma5AndLessKeywords + String::from(" export import"),
    Six = ecma5AndLessKeywords + String::from(" const class extends export import super"),
}
