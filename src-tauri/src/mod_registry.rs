use std::io;
use winreg::RegKey;

fn get_user_variable_path() -> io::Result<String> {
    let hkey_current_user = RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let user_environment = hkey_current_user.open_subkey("Environment")?;
    let path: String = user_environment.get_value("Path")?;
    Ok(path)
}

fn get_system_variable_path() -> io::Result<String> {
    let hkey_local_machine = RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
    let system_environment = hkey_local_machine.open_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment")?;
    let path: String = system_environment.get_value("Path")?;
    Ok(path)
}


pub fn get_user_variables() -> Vec<String> {
    let user_path_result = get_user_variable_path();
    let mut user_path_data = Vec::new();
    match user_path_result {
        Ok(total_path) => {
            for path in total_path.split(";") {
                user_path_data.push(path.to_string());
            }
            return user_path_data;
        },
        Err(_err) => {
            return user_path_data;
        }
    }
}

pub fn save_user_variables(user_path: String) -> io::Result<()> {
    let hkey_current_user = RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let (environment, _disp) = hkey_current_user.create_subkey("Environment")?;
    return environment.set_value("Path", &user_path);
}

pub fn get_system_variables() -> Vec<String> {
    let system_path_result = get_system_variable_path();
    let mut system_path_data = Vec::new();
    match system_path_result {
        Ok(total_path) => {
            for path in total_path.split(";") {
                system_path_data.push(path.to_string());
            }
            return system_path_data;
        },
        Err(_err) => {
            return system_path_data;
        }
    }
}

pub fn save_system_variables(system_path: String) -> io::Result<()> {
    let hkey_local_machine = RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
    let (environment, _disp) = hkey_local_machine.create_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment")?;
    return environment.set_value("Path", &system_path);
}