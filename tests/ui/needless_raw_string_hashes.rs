//@run-rustfix
#![allow(clippy::needless_raw_string, clippy::no_effect, unused)]
#![warn(clippy::needless_raw_string_hashes)]
#![feature(c_str_literals)]

fn main() {
    r#"aaa"#;
    r##"Hello "world"!"##;
    r######" "### "## "# "######;
    r######" "aa" "# "## "######;
    br#"aaa"#;
    br##"Hello "world"!"##;
    br######" "### "## "# "######;
    br######" "aa" "# "## "######;
    cr#"aaa"#;
    cr##"Hello "world"!"##;
    cr######" "### "## "# "######;
    cr######" "aa" "# "## "######;
}
