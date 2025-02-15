// Copyright 2021 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR BSD-3-Clause

use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, PartialEq)]
pub enum Error {
    DownloadError(String),
}

/// Downloads from S3 the first resource that match the parameters:
///     - `r_type`: the resource type; e.g. "kernel", "disk".
///     - `r_tags`: optional tags to filter the resources; e.g. "{\"halt-after-boot\": true}"
pub fn s3_download(r_type: &str, r_tags: Option<&str>) -> Result<PathBuf, Error> {
    // The function downloads the resource from S3 which matched the paramaters. The first parameter of type string contains the resource
    // type. The second parameter is optional and contains additional parameters for filtering.
    
    // dld_script is the command sent to the function in the file
    // format used for interpolation to create a string command
    let dld_script = format!(
        "{}/../../tests/tools/s3_download.py",
        env!("CARGO_MANIFEST_DIR")
    );

    // command spawns a new process which runs the python script for downloading the resource from S3
    // dld_script along with args are used for configuring the process
    // the python script is available at ../../tests/tools/s3_download.py
    let output = Command::new(dld_script.as_str())
        .arg("-t")
        .arg(r_type)
        .arg("--tags")
        .arg(r_tags.unwrap_or("{}"))
        .arg("-1")
        .output()
        .expect("failed to execute process");
    
    // if the code returns some error, the rust returns the same error
    if !output.status.success() {
        return Err(Error::DownloadError(
            String::from_utf8(output.stderr).unwrap(),
        ));
    }
    
    // else the script returns the path to the downloaded resource
    let res: String = String::from_utf8(output.stdout)
        .unwrap()
        .split('\n')
        .map(String::from)
        .next()
        .ok_or_else(|| Error::DownloadError(String::from("Not found.")))?;
    Ok(PathBuf::from(res))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_cases() {
        assert!(matches!(
            s3_download("", None).unwrap_err(),
            Error::DownloadError(e) if e.contains("Missing required parameter")
        ));

        assert!(matches!(
            s3_download("random", None).unwrap_err(),
            Error::DownloadError(e) if e.contains("No resources found")
        ));
    }
}
