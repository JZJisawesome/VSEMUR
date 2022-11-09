pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

//Thanks https://stackoverflow.com/questions/67307526/is-it-possible-to-get-the-cargo-environment-variable-information-for-one-crate-u

pub fn vsemur_version_major() -> u128 {
    return env!("CARGO_PKG_VERSION_MAJOR").parse::<u128>().unwrap();
}

pub fn vsemur_version_minor() -> u128 {
    return env!("CARGO_PKG_VERSION_MINOR").parse::<u128>().unwrap();
}

pub fn vsemur_version_patch() -> u128 {
    return env!("CARGO_PKG_VERSION_PATCH").parse::<u128>().unwrap();
}

pub fn vsemur_version_string() -> String {
    return format!("libvsemur v{}.{}.{}", env!("CARGO_PKG_VERSION_MAJOR"), env!("CARGO_PKG_VERSION_MINOR"), env!("CARGO_PKG_VERSION_PATCH"));
}
