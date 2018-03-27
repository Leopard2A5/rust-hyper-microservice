use schema::urls;

#[derive(Debug, Queryable)]
pub struct Url {
    pub short: String,
    pub long: String,
}

#[derive(Debug, Insertable)]
#[table_name="urls"]
pub struct NewUrl<'a> {
    pub short: &'a str,
    pub long: &'a str,
}
