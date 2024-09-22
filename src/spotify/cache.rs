use std::fs::File;
use std::io::Write;

use tempfile::NamedTempFile;

// In order to know which songs are not in the liked playlist. I would like to write all the songs
// and their ids to a temp file,labeled with the endpoint. If this file exists then I dont make a
// request to create it. Otherwise I'll need to. Then I can compare which songs are in starred but
// not in the playlist... expensive
// getUsersSaveTracks
//  [id]
//      21312312321
//  [name]
//      "song name here"

pub fn write_spotify_result_to_temp_file(buffer: &[u8]) -> Option<File> {
    match tempfile::tempfile() {
        Ok(mut file) => match file.write_all(buffer) {
            Ok(_) => Some(file),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

const TEMP_NAME_PREFIX: &str = "echo-";
pub fn create_app_temp_file(prefix: &str) -> Option<NamedTempFile> {
    let app_named_prefix = format!("{}{}", TEMP_NAME_PREFIX, prefix);
    match tempfile::Builder::new()
        .prefix(app_named_prefix.as_str())
        .tempfile()
    {
        Ok(named_file) => Some(named_file),
        Err(..) => None,
    }
}
