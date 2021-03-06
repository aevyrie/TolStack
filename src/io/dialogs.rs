use nfd::Response;
use std::io;
use std::path::{Path, PathBuf};

pub async fn open() -> Result<PathBuf, io::Error> {
    let result: nfd::Response = match async { nfd::open_file_dialog(Some("json"), None) }.await {
        Ok(result) => result,
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unable to unwrap data from new file dialog",
            ))
        }
    };

    let file_string: String = match result {
        Response::Okay(file_path) => file_path,
        Response::OkayMultiple(_) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Multiple files returned when one was expected",
            ))
        }
        Response::Cancel => {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "User cancelled file open",
            ))
        }
    };

    let mut result: PathBuf = PathBuf::new();
    result.push(Path::new(&file_string));

    if result.exists() {
        return Ok(result);
    } else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File does not exist",
        ));
    }
}

pub async fn save_as() -> Result<PathBuf, io::Error> {
    let result: nfd::Response = match async { nfd::open_save_dialog(Some("json"), None) }.await {
        Ok(result) => result,
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unable to unwrap data from new file dialog",
            ))
        }
    };

    let file_string: String = match result {
        Response::Okay(file_path) => file_path,
        Response::OkayMultiple(_) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Multiple files returned when one was expected",
            ))
        }
        Response::Cancel => {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "User cancelled file open",
            ))
        }
    };

    let mut result: PathBuf = PathBuf::new();
    result.push(Path::new(&file_string));

    if match result.parent() {
        Some(parent) => parent.exists(),
        None => false,
    } {
        return Ok(result);
    } else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Parent directory does not exist",
        ));
    }
}
