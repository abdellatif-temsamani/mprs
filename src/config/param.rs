#[derive(Debug)]
pub struct Param {
    pub flag: String,
    pub value: String,
}

impl Param {
    pub fn command(value: String) -> Self {
        Self {
            flag: "MPD".to_owned(),
            value,
        }
    }

    pub fn config(flag: String, value: String) -> Self {
        Self { flag, value }
    }
}
