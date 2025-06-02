use crate::configuration::utils::error_util::FileError;
use std::fs::{File, ReadDir};
use std::path::{Path, PathBuf};
use std::{env, fs};

/**
 * @description: 获取文件夹列表
 * @author: illya
 * @date: 2025/5/14 15:53
 **/
pub fn acquire_file_list(path: &str) -> Result<ReadDir, FileError> {
    match fs::read_dir(path) {
        Ok(files) => Ok(files),
        Err(err) => Err(FileError::FileDirNotFound(err)),
    }
}

/**
 * @description: 获取文件内容
 * @author: illya
 * @date: 2025/5/14 15:59
 **/
pub fn read_file_content(path: PathBuf) -> Result<String, FileError> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(err) => Err(FileError::FileReadError(err)),
    }
}

/**
 * @description: 将str转化为Path
 * @author: illya
 * @date: 2025/5/14 19:10
 **/
pub fn acquire_file_path(file_path: &str) -> &Path {
    Path::new(file_path)
}

/**
 * @description: 判断文件是否存在
 * @author: illya
 * @date: 2025/5/14 18:51
 **/
pub fn file_valid(file_path: &str) -> bool {
    // 如果文件存在
    Path::exists(acquire_file_path(file_path))
}

/**
 * @description: 文件夹
 * @author: illya
 * @date: 2025/5/14 18:54
 **/
pub fn create_file(database_path: &Path) -> Result<(), FileError> {
    // 如果存在父文件路径，且未创建，则创建父文件路径
    if let Some(parent_file_path) = database_path.parent() {
        fs::create_dir_all(parent_file_path).map_err(|err| FileError::FileDirCreateError(err))?;
    }
    // 创建文件
    File::create(&database_path).map_err(|err| FileError::FileCreateError(err))?;
    Ok(())
}

/**
 * @description: 获取数据库url
 * @author: illya
 * @date: 2025/5/22 15:40
 **/
pub fn acquire_database_url() -> String {
    env::var("DATABASE_FILE_ADDRESS").unwrap_or(String::from("./resources/database.sqlite"))
}

/**
 * @description: 移除某个文件
 * @author: illya
 * @date: 2025/6/1 15:41
 **/
pub fn delete_file(database_path: &Path) -> Result<(), FileError> {
    fs::remove_file(database_path).map_err(|err| FileError::FileDeleteError(err))
}
