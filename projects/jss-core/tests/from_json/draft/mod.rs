use json_value::serde_json::Value;
use jss_convert::JssSchema;
use jss_error::Result;
use std::{io::Write, path::PathBuf};

#[test]
fn convert_all_draft() -> Result<()> {
    let mut drafts = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    drafts.push("tests/from_json/draft");

    for file in drafts.read_dir()? {
        let mut path = file?.path();
        if let Some(s) = path.extension() {
            if s.eq("json") {
                let json: Value = std::fs::read_to_string(&path)?.parse()?;
                let jss = JssSchema::parse_json_schema_as_jss(json)?;
                path.set_extension("jss");
                let mut f = std::fs::OpenOptions::new().write(true).create(true).open(path)?;
                f.write_all(jss.as_bytes())?;
                f.flush()?;
            }
        }
    }
    Ok(())
}
