pub fn opt_display<T>(opt: &Option<T>, default: &str) -> String
where
    T: std::fmt::Display,
{
    opt.as_ref()
        .map(|v| v.to_string())
        .unwrap_or_else(|| default.to_string())
}

pub fn opt_debug<T>(opt: &Option<T>, default: &str) -> String
where
    T: std::fmt::Debug,
{
    opt.as_ref()
        .map(|v: &T| format!("{:?}", v))
        .unwrap_or_else(|| default.to_string())
}
