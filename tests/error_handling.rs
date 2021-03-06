use crate::common::TestEnv;
use assert_fs::fixture::ChildPath;
use assert_fs::prelude::*;
use pier::{error::*, script::Script, Pier};

// Tests that it returns the error AliasNotFound if the alias given does not exist
pier_test!(lib => test_error_alias_not_found, cfg => r#"
[scripts.test_cmd_1]
alias = 'test_cmd_1'
command = 'echo test_1' 
"#, | _cfg: ChildPath, mut lib: Pier | {
    err_eq!(lib.remove_script("non_existant"), AliasNotFound);
    err_eq!(lib.fetch_script("non_existant"), AliasNotFound);
});

// Tests that it returns the error NoScriptsExists if there is no scripts in the config
pier_test!(lib => test_error_no_scripts_exists, cfg => r#""#,
| _cfg: ChildPath, mut lib: Pier | {
    err_eq!(lib.remove_script(""), NoScriptsExists);
    err_eq!(lib.fetch_script(""), NoScriptsExists);
    err_eq!(lib
        .list_scripts(None, false, None), NoScriptsExists);
});

pier_test!(lib => test_error_alias_already_exists, cfg => r#"
[scripts.test_cmd_1]
alias = 'test_cmd_1'
command = 'echo test_1' 
"#, | _cfg: ChildPath, mut lib: Pier | {
    let script = Script {
    alias: "test_cmd_1".to_string(),
    command: "echo something else".to_string(),
    description: None,
    reference: None,
    tags: None
    };
    err_eq!(lib.add_script(script, false), AliasAlreadyExists);
});

pier_test!(lib => test_error_alias_already_exists_on_move, cfg => r#"
[scripts.test_cmd_1]
alias = 'test_cmd_1'
command = 'echo test_1'

[scripts.test_cmd_2]
alias = 'test_cmd_2'
command = 'echo test_2'
"#, | _cfg: ChildPath, mut lib: Pier | {
    err_eq!(lib.move_script("test_cmd_1", "test_cmd_2",  false), AliasAlreadyExists);
});

// Tests that it returns the error ConfigRead if the file cannot be read.
// In this case the file is not created
pier_test!(basic => test_config_read_error, | te: TestEnv | {
    let path = te.join_root("non_existant_file");
    let lib = Pier::from(Some(path), false);
    err_eq!(lib, ConfigRead);
});

// Tests that it returns the error ConfigWrite if the file cannot be written to
// In this case the file is not created
pier_test!(basic => test_config_write_error, | _te: TestEnv | {
    let lib = Pier::new();
    err_eq!(lib.write(), ConfigWrite);
});

// Tests that it returns the error TomlParse if the config is not valid Toml
pier_test!(basic => test_toml_parse_error, | te: TestEnv| {
    let cfg = te.dir.child("pier_config");
    cfg.touch().expect("Unable to create file");
    cfg.write_str(trim!(r#"
        [scripts.test_cmd_1]
        alias = 'test_cmd_1'
        command = echo test_1 
        "#)
    ).expect("Unable to write to file");

    let lib = Pier::from_file(cfg.path().to_path_buf(), false);
    err_eq!(lib, TomlParse);
});
