use std::io::{BufRead, Seek};

pub struct Watcher {
    path: std::path::PathBuf,
    pos: u64,
    reader: std::io::BufReader<std::fs::File>,
    create_time: std::time::SystemTime, // store create time, if this changes it means we have a new log file
}

impl Watcher {
    pub fn register(path: std::path::PathBuf) -> Result<Watcher, std::io::Error> {
        let file = match std::fs::File::open(&path) {
            Ok(f) => f,
            Err(err) => return Err(err),
        };

        let metadata = match file.metadata() {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        let mut reader = std::io::BufReader::new(file);

        let pos = metadata.len();
        reader.seek(std::io::SeekFrom::Start(pos))?;

        let create_time = match metadata.created() {
            Ok(time) => time,
            Err(err) => return Err(err),
        };

        Ok(Watcher {
            path,
            pos,
            reader,
            create_time,
        })
    }

    fn reregister_on_rotation(&mut self) -> Result<(), std::io::Error> {
        let file = match std::fs::File::open(&self.path) {
            Ok(f) => f,
            Err(err) => return Err(err),
        };

        let metadata = match file.metadata() {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        let create_time = match metadata.created() {
            Ok(time) => time,
            Err(err) => return Err(err),
        };

        if create_time != self.create_time {
            self.reader = std::io::BufReader::new(file);
            self.pos = 0;
            self.create_time = create_time;
        }

        Ok(())
    }   

    pub fn watch<F>(&mut self, action: F) -> Result<(), std::io::Error>
    where
        F: Fn(&str),
    {
        loop {
            let mut line = String::new();
            let bytes_read = self.reader.read_line(&mut line);
            match bytes_read {
                Ok(bytes) => {
                    if bytes > 0 {
                        self.pos += bytes as u64;
                        self.reader.seek(std::io::SeekFrom::Start(self.pos))?;

                        action(&line);
                    } else {
                        if let Err(err) = self.reregister_on_rotation() {
                            return Err(err);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("{}", err);
                }
            }
        }
    }
}
