fn main() -> Result<(), chat_usage_db::error::AppError> {
    match chat_usage_db::cli::run() {
        Ok(()) => {
            println!("\x1b[32m[âœ“] Operation completed successfully.\x1b[0m");
            Ok(())
        }
        Err(err) => {
            eprintln!("\x1b[31m[âœ—] Error: {}\x1b[0m", err);
            Err(err)
        }
    }
}