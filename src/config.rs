#[derive(Debug)]
pub struct Config<'a> {
    pub name: &'a str,
    pub data_endpoint_url: &'a str,
    pub cache_file_path: &'a str,
}
