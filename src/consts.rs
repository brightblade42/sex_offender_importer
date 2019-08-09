use std::str;

const CREATE_REMOTE_FILE_LIST:String  = String::from(
    r#" CREATE TABLE if not exists
        remote_file_list (rpath, name, last_modified, size integer, status) ",
    "# );

