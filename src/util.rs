// Does this currently based on file extension
// In the future it may parse the header / beginning of the file in question
pub fn is_supported_format(path: &str) -> bool {
    let extension = path.split(".").last().unwrap().to_lowercase();

    let supported: Vec<String> = vec!["jpg", "jpeg", "gif", "png", "tif", "tiff"].iter().map(|s| s.to_string()).collect();

    supported.contains(&extension)
}
