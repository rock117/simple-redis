mod file;
mod mem;

trait Storage {
    fn put<T>(key: String, value: T);
    fn get<T>(key: &str) -> Option<T>;

    fn remove(key: &str);
}
