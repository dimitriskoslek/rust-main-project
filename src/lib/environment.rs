use std::env;

pub fn get_environment_variables() -> EnvVars {

    dotenv::from_filename("/home/dimitriskl/Programming/sociality/src/.env").ok();
    
    let db_url = env::var("DATABASE_URL");
    let db_user_username = env::var("DATABASE_USER_USERNAME");
    let db_user_password = env::var("DATABASE_USER_PASSWORD");

    if !db_url.is_ok(){
        println!("Env var for the DB url was not found");
        std::process::exit(1);
    }
    if !db_user_username.is_ok(){
        println!("Env var for the DB user username was not found");
        std::process::exit(1);
    }
    if !db_user_password.is_ok(){
        println!("Env var for the DB user password was not found");
        std::process::exit(1);
    }

    let envs = EnvVars {
        db_url: db_url.unwrap(),
        db_user_username: db_user_username.unwrap(),
        db_user_password: db_user_password.unwrap(),
    };

    envs
}


pub struct EnvVars {
    pub db_url: String,
    pub db_user_username: String,
    pub db_user_password: String,
}











    
 
    