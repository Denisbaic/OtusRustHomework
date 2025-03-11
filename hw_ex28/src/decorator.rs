use std::io::Read;

use tokio::{fs::File, io::AsyncReadExt};

trait ReaderAsync<T, E> {
    async fn read(&mut self) -> Result<T, E>;
}
/////////////////Static Decorator Begin/////////////
struct StaticLoggingReaderAsync<T>
where
    T: ReaderAsync<String, std::io::Error>,
{
    inner: T,
}

impl<T> StaticLoggingReaderAsync<T>
where
    T: ReaderAsync<String, std::io::Error>,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> ReaderAsync<String, std::io::Error> for StaticLoggingReaderAsync<T>
where
    T: ReaderAsync<String, std::io::Error>,
{
    async fn read(&mut self) -> Result<String, std::io::Error> {
        println!("Reading data from file");
        self.inner.read().await
    }
}

struct FileReaderAsync {
    file_instance: tokio::fs::File,
}

impl FileReaderAsync {
    pub fn new(file_instance: File) -> Self {
        Self { file_instance }
    }
}

impl ReaderAsync<String, std::io::Error> for FileReaderAsync {
    async fn read(&mut self) -> Result<String, std::io::Error> {
        let mut buf = String::new();
        let read_result = self.file_instance.read_to_string(&mut buf).await;

        read_result.map(|_| buf)
    }
}

/////////////////Static Decorator End/////////////

/////////////////Dynamic Decorator Begin/////////////
trait Reader<T, E> {
    fn read(&mut self) -> Result<T, E>;
}

struct FileReader {
    file_instance: std::fs::File,
}

impl FileReader {
    pub fn new(file_instance: std::fs::File) -> Self {
        Self { file_instance }
    }
}

impl Reader<String, std::io::Error> for FileReader {
    fn read(&mut self) -> Result<String, std::io::Error> {
        let mut buf = String::new();
        let read_result = self.file_instance.read_to_string(&mut buf);

        read_result.map(|_| buf)
    }
}

struct DynamicLoggingReader<T, E> {
    inner: Box<dyn Reader<T, E>>,
}

impl<T, E> DynamicLoggingReader<T, E> {
    pub fn new(inner: Box<dyn Reader<T, E>>) -> Self {
        Self { inner }
    }
}

impl<T, E> Reader<T, E> for DynamicLoggingReader<T, E> {
    fn read(&mut self) -> Result<T, E> {
        println!("Reading data from file");
        self.inner.read()
    }
}

///////////////////Dynamic Decorator End/////////////

#[cfg(test)]
mod tests {
    use crate::decorator::{
        DynamicLoggingReader, FileReader, FileReaderAsync, ReaderAsync, StaticLoggingReaderAsync,
    };

    use super::Reader;

    #[tokio::test]
    async fn static_decorator_test() {
        let file_instance = tokio::fs::File::open("test.txt").await.unwrap();
        let mut logging_reader = StaticLoggingReaderAsync::new(FileReaderAsync::new(file_instance));
        assert_eq!(logging_reader.read().await.unwrap(), "Hello World!");
    }

    #[test]
    fn dynamic_decorator_test() {
        let file_instance = std::fs::File::open("test.txt").unwrap();
        let mut logging_reader =
            DynamicLoggingReader::new(Box::new(FileReader::new(file_instance)));
        assert_eq!(logging_reader.read().unwrap(), "Hello World!");
    }
}
