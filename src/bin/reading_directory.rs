use std::{fs, io};

fn main() -> io::Result<()> {
    for entry_result in fs::read_dir("./")? {
        let entry = entry_result?;
        let entryname = entry.path();
        let metadata = fs::metadata(&entryname)?;
        if metadata.is_file() {
            println!("{entryname:?}\n----------");
            let contents = fs::read_to_string(&entryname)?;
            println!("{contents}");
        }
    }

    Ok(())
}
