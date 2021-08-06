use Organization::ParseSettingsModule;
use std::path::Path;

pub struct SettingsFile {
    pub file: String,
}

impl SettingsFile {
    pub fn new(file: String) -> Self {
        SettingsFile {
            file: match std::fs::read_to_string(file) {
                Ok(t) => t,
                Err(_) => panic!("Game files corrupted. Please verify files using Steam"),
            },
        }
    }
    pub fn display_manifest(&self) {
        println!("{:?}", &self.file);
    }

    pub fn make_json(&self, dir: &str) {
        let parsed = SettingsFile::words_by_line(&self.file);

        let mut total_blocks: Vec<Vec<&str>> = vec![];
        let mut sub_blocks: Vec<&str> = vec![];
        for v in parsed {
            for v1 in v {
                //Not putting "file" in becasue it doesn't have logic trigger
                if v1 != "" {
                    sub_blocks.push(v1);
                } else {
                    total_blocks.push(sub_blocks.to_vec());
                    sub_blocks.clear();
                }
            } 
        }
        total_blocks.push(sub_blocks.to_vec());

        println!("{:?}", total_blocks);
        let segments: Vec<ParseSettingsModule> = vec![];
        for bit in total_blocks {
            // segments.push(ParseSettingsModule::new(bit, &dir));
        }

        
    }

    fn words_by_line<'a>(file: &'a str) -> Vec<Vec<&'a str>> {
        file.lines().map(|l| {
            l.split("\n").collect()
        }).collect()
    }

    // todo!("https://doc.rust-lang.org/stable/book/ch04-03-slices.html");
}

pub mod Organization{
    enum TargetType {
        Directory,
        File
    }
    
    pub struct ParseSettingsModule {
        target: TargetType,
        path: String,
        sub_dir: bool
        
    }
    
    impl ParseSettingsModule {
        pub fn new(target: Vec<&str>, dir: &str) -> Self {
            let mut targ: Option<TargetType> = None;
            if target[0] == "directory" {
                targ = Some(TargetType::Directory);
            } else if target[0] == "file" {
                targ = Some(TargetType::File);
            }

            Self {
                target: match targ {
                    Some(TargetType::Directory) => TargetType::Directory,
                    Some(TargetType::File) => TargetType::File,
                },
                path: format!("{}/{}"),

            }
        }
    }
}

todo!("Implement struct for each field in settings file so no more parsing and we can begin checksum work");

