use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account<'a> {
    #[serde(borrow, default)]
    pub name: Option<&'a str>,

    #[serde(borrow, default)]
    pub email: Option<&'a str>,

    pub username: &'a str,
}
