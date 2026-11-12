#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum SortedFolders {
    Images,
    Documents,
    Archives,
    Misc,
}

impl SortedFolders {
    pub fn folder_name(&self) -> &str {
        match self {
            SortedFolders::Images => "Images",
            SortedFolders::Documents => "Documents",
            SortedFolders::Archives => "Archives",
            SortedFolders::Misc => "Misc",
        }
    }
}

pub fn categorise(ext: &str) -> SortedFolders {
    match ext {
        "jpg" | "png" => SortedFolders::Images,
        "doc" | "docx" | "pdf" => SortedFolders::Documents,
        "zip" | "7z" | "rar" => SortedFolders::Archives,
        _ => SortedFolders::Misc,
    }
}
