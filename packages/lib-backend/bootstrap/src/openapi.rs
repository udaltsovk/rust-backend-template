use std::io;

use utoipa::openapi::OpenApi;

#[derive(thiserror::Error, Debug)]
pub enum OpenAPISaverError {
    #[error("Failed to serialize OpenAPI: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Failed to save OpenAPI: {0}")]
    Save(#[from] io::Error),
}

pub type OpenAPISaverResult = Result<(), OpenAPISaverError>;

pub trait FileWriter {
    fn write(&self, path: &str, content: &str) -> Result<(), io::Error>;
}

pub struct DefaultFileWriter;

impl FileWriter for DefaultFileWriter {
    fn write(&self, path: &str, content: &str) -> Result<(), io::Error> {
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)
    }
}

pub struct OpenAPISaver<W: FileWriter> {
    writer: W,
}

impl<W: FileWriter> OpenAPISaver<W> {
    pub const fn new(writer: W) -> Self {
        Self {
            writer,
        }
    }

    pub fn save_as(&self, api: &OpenApi, name: &str) -> OpenAPISaverResult {
        let openapi_json = api.to_pretty_json()?;

        let path = format!("./assets/openapi/{name}.json");
        self.writer.write(&path, &openapi_json)?;

        Ok(())
    }
}

pub trait OpenAPISaverTrait {
    fn save_as(&self, name: &str) -> OpenAPISaverResult;
}

impl OpenAPISaverTrait for OpenApi {
    fn save_as(&self, name: &str) -> OpenAPISaverResult {
        let saver = OpenAPISaver::new(DefaultFileWriter);
        saver.save_as(self, name)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        fs,
        path::Path,
        sync::{Arc, Mutex},
    };

    use utoipa::OpenApi as _;

    use super::*;

    // Mock writer for testing
    #[derive(Clone)]
    struct MockFileWriter {
        files: Arc<Mutex<HashMap<String, String>>>,
        should_fail: bool,
        fail_on_path: Option<String>,
    }

    impl MockFileWriter {
        fn new() -> Self {
            Self {
                files: Arc::new(Mutex::new(HashMap::new())),
                should_fail: false,
                fail_on_path: None,
            }
        }

        fn new_failing() -> Self {
            Self {
                files: Arc::new(Mutex::new(HashMap::new())),
                should_fail: true,
                fail_on_path: None,
            }
        }

        fn new_failing_on_path(path: &str) -> Self {
            Self {
                files: Arc::new(Mutex::new(HashMap::new())),
                should_fail: false,
                fail_on_path: Some(path.to_string()),
            }
        }

        #[expect(dead_code, reason = "Utility method for debugging tests")]
        fn get_files(&self) -> HashMap<String, String> {
            self.files
                .lock()
                .expect("Failed to lock mock files")
                .clone()
        }

        fn has_file(&self, path: &str) -> bool {
            self.files
                .lock()
                .expect("Failed to lock mock files")
                .contains_key(path)
        }

        fn get_file_content(&self, path: &str) -> Option<String> {
            self.files
                .lock()
                .expect("Failed to lock mock files")
                .get(path)
                .cloned()
        }

        fn file_count(&self) -> usize {
            self.files.lock().expect("Failed to lock mock files").len()
        }
    }

    impl FileWriter for MockFileWriter {
        fn write(&self, path: &str, content: &str) -> Result<(), io::Error> {
            if self.should_fail
                || self.fail_on_path.as_ref() == Some(&path.to_string())
            {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "Mock error",
                ));
            }
            self.files
                .lock()
                .map_err(|_| io::Error::other("Failed to lock mock files"))?
                .insert(path.to_string(), content.to_string());
            Ok(())
        }
    }

    // Test OpenAPI struct
    #[derive(utoipa::OpenApi)]
    #[openapi(
        info(
            title = "Test API",
            version = "1.0.0",
            description = "Test API for unit testing"
        ),
        paths(),
        components()
    )]
    struct TestApi;

    #[derive(utoipa::OpenApi)]
    #[openapi(
        info(
            title = "Another Test API",
            version = "2.0.0",
            description = "Another API for testing different specs"
        ),
        paths(),
        components()
    )]
    struct AnotherTestApi;

    #[test]
    fn openapi_saver_new_creates_saver() {
        let mock_writer = MockFileWriter::new();
        let saver = OpenAPISaver::new(mock_writer.clone());

        // Verify the saver can be used
        let api = TestApi::openapi();
        let result = saver.save_as(&api, "test_new");

        assert!(result.is_ok(), "Newly created saver should work");
        assert!(mock_writer.has_file("./assets/openapi/test_new.json"));
        assert_eq!(mock_writer.file_count(), 1);
    }

    #[test]
    fn openapi_saver_save_as_success() {
        let mock_writer = MockFileWriter::new();
        let saver = OpenAPISaver::new(mock_writer.clone());
        let api = TestApi::openapi();

        let result = saver.save_as(&api, "test_api");

        assert!(result.is_ok(), "Save operation should succeed");
        assert!(mock_writer.has_file("./assets/openapi/test_api.json"));

        let content = mock_writer
            .get_file_content("./assets/openapi/test_api.json")
            .expect("File should exist after save");
        assert!(content.contains("Test API"));
        assert!(content.contains("1.0.0"));
        assert!(content.contains("Test API for unit testing"));
    }

    #[test]
    fn openapi_saver_save_as_io_error() {
        let mock_writer = MockFileWriter::new_failing();
        let saver = OpenAPISaver::new(mock_writer);
        let api = TestApi::openapi();

        let result = saver.save_as(&api, "test_api");

        assert!(result.is_err(), "Save should fail with failing writer");
        match result.expect_err("Should have failed") {
            OpenAPISaverError::Save(e) => {
                assert_eq!(e.kind(), io::ErrorKind::PermissionDenied);
            },
            _ => panic!("Expected Save error"),
        }
    }

    #[test]
    fn openapi_saver_different_names() {
        let mock_writer = MockFileWriter::new();
        let saver = OpenAPISaver::new(mock_writer.clone());
        let api = TestApi::openapi();

        let names =
            vec!["api1", "api-2", "api_3", "api123", "complex-name_with.dots"];

        for name in &names {
            let result = saver.save_as(&api, name);
            assert!(result.is_ok(), "Failed to save OpenAPI with name: {name}");

            let expected_path = format!("./assets/openapi/{name}.json");
            assert!(
                mock_writer.has_file(&expected_path),
                "File should exist for name: {name}"
            );
        }

        assert_eq!(mock_writer.file_count(), names.len());

        // Verify all files have correct content
        for name in &names {
            let expected_path = format!("./assets/openapi/{name}.json");
            let content = mock_writer
                .get_file_content(&expected_path)
                .expect("File should exist");
            assert!(
                content.contains("Test API"),
                "Content should be correct for {name}"
            );
        }
    }

    #[test]
    fn openapi_saver_overwrites_file() {
        let mock_writer = MockFileWriter::new();
        let saver = OpenAPISaver::new(mock_writer.clone());
        let api1 = TestApi::openapi();
        let api2 = AnotherTestApi::openapi();

        // Save first API
        let result1 = saver.save_as(&api1, "overwrite_test");
        assert!(result1.is_ok(), "First save should succeed");

        let content1 = mock_writer
            .get_file_content("./assets/openapi/overwrite_test.json")
            .expect("File should exist after first save");
        assert!(content1.contains("Test API"));
        assert!(content1.contains("1.0.0"));

        // Save second API with same name (should overwrite)
        let result2 = saver.save_as(&api2, "overwrite_test");
        assert!(result2.is_ok(), "Second save should succeed");

        let content2 = mock_writer
            .get_file_content("./assets/openapi/overwrite_test.json")
            .expect("File should exist after second save");
        assert!(content2.contains("Another Test API"));
        assert!(content2.contains("2.0.0"));
        assert!(!content2.contains("Test API for unit testing"));

        // Should still only have one file
        assert_eq!(mock_writer.file_count(), 1);
    }

    #[test]
    fn openapi_json_formatting() {
        let mock_writer = MockFileWriter::new();
        let saver = OpenAPISaver::new(mock_writer.clone());
        let api = TestApi::openapi();

        let result = saver.save_as(&api, "format_test");
        assert!(result.is_ok(), "Save should succeed");

        let content = mock_writer
            .get_file_content("./assets/openapi/format_test.json")
            .expect("File should exist after save");

        // Verify it's pretty-formatted JSON
        assert!(content.contains('\n'), "JSON should have newlines");
        assert!(content.contains("  "), "JSON should have indentation");

        // Verify it's valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&content)
            .expect("Content should be valid JSON");
        assert!(parsed.is_object(), "Parsed JSON should be an object");

        // Verify specific structure
        let obj = parsed.as_object().expect("Should be object");
        assert!(obj.contains_key("openapi"), "Should have openapi version");
        assert!(obj.contains_key("info"), "Should have info section");
    }

    #[test]
    fn error_types_serialization() {
        // Test serialization error (hard to trigger with valid OpenApi, so we test the error type)
        let json_error = serde_json::Error::io(io::Error::new(
            io::ErrorKind::InvalidData,
            "Test JSON error",
        ));
        let openapi_error = OpenAPISaverError::Serialization(json_error);
        let error_msg = format!("{openapi_error}");
        assert!(error_msg.contains("Failed to serialize OpenAPI"));
    }

    #[test]
    fn error_types_save() {
        let io_error =
            io::Error::new(io::ErrorKind::NotFound, "File not found");
        let save_error = OpenAPISaverError::Save(io_error);
        let error_msg = format!("{save_error}");
        assert!(error_msg.contains("Failed to save OpenAPI"));
        assert!(error_msg.contains("File not found"));

        // Test From conversion for io::Error
        let io_error2 = io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Permission denied",
        );
        let converted_error: OpenAPISaverError = io_error2.into();
        match converted_error {
            OpenAPISaverError::Save(e) => {
                assert_eq!(e.kind(), io::ErrorKind::PermissionDenied);
            },
            _ => panic!("Expected Save error"),
        }
    }

    #[test]
    fn result_type_alias_usage() {
        fn returns_ok() {
            // Test function that doesn't need to return Result
        }

        fn returns_error() -> OpenAPISaverResult {
            Err(OpenAPISaverError::Save(io::Error::other("Test error")))
        }

        returns_ok();
        assert!(returns_error().is_err());
    }

    #[test]
    fn default_file_writer_functionality() {
        let writer = DefaultFileWriter;
        let test_path = "./test_default_writer_output.txt";
        let test_content = "Test content for DefaultFileWriter";

        // Test successful write
        let result = writer.write(test_path, test_content);
        assert!(
            result.is_ok(),
            "DefaultFileWriter should write successfully"
        );

        // Verify file exists and has correct content
        assert!(
            Path::new(test_path).exists(),
            "File should exist after write"
        );
        let read_content = fs::read_to_string(test_path)
            .expect("Should be able to read written file");
        assert_eq!(
            read_content, test_content,
            "File content should match what was written"
        );

        // Test that writer can handle the file writer trait
        let trait_result: Result<(), io::Error> =
            writer.write(test_path, "updated content");
        trait_result.expect("Should successfully write to test path");

        let updated_content =
            fs::read_to_string(test_path).expect("Should read updated file");
        assert_eq!(updated_content, "updated content");

        // Cleanup
        drop(fs::remove_file(test_path));
    }

    #[test]
    fn default_file_writer_io_error() {
        let writer = DefaultFileWriter;
        let invalid_path = "/invalid/path/that/should/not/exist/test.txt";

        // Test write to invalid path
        let result = writer.write(invalid_path, "test content");
        assert!(
            result.is_err(),
            "DefaultFileWriter should fail with invalid path"
        );

        // Verify it's an IO error (could be NotFound or PermissionDenied depending on system)
        match result {
            Err(e) => assert!(
                e.kind() == io::ErrorKind::NotFound
                    || e.kind() == io::ErrorKind::PermissionDenied,
                "Expected NotFound or PermissionDenied error, got: {:?}",
                e.kind()
            ),
            Ok(()) => panic!("Expected IO error"),
        }
    }

    #[test]
    fn default_file_writer_overwrite() {
        let writer = DefaultFileWriter;
        let test_path = "./test_overwrite.txt";
        let initial_content = "Initial content";
        let new_content = "New content";

        // Write initial content
        let result1 = writer.write(test_path, initial_content);
        assert!(result1.is_ok(), "First write should succeed");

        // Verify initial content
        let read_content1 =
            fs::read_to_string(test_path).expect("Should read initial content");
        assert_eq!(read_content1, initial_content);

        // Overwrite with new content
        let result2 = writer.write(test_path, new_content);
        assert!(result2.is_ok(), "Overwrite should succeed");

        // Verify new content
        let read_content2 =
            fs::read_to_string(test_path).expect("Should read new content");
        assert_eq!(read_content2, new_content);
        assert_ne!(
            read_content2, initial_content,
            "Content should have changed"
        );

        // Cleanup
        drop(fs::remove_file(test_path));
    }

    #[test]
    fn default_file_writer_with_nested_directories() {
        let writer = DefaultFileWriter;
        let nested_path = "./test_dir/nested/deep/test_file.txt";
        let content = "Testing nested directory creation";

        // Remove test directory if it exists
        if Path::new("./test_dir").exists() {
            drop(fs::remove_dir_all("./test_dir"));
        }

        // Write to nested path
        let result = writer.write(nested_path, content);
        assert!(
            result.is_ok(),
            "Should create nested directories and write file"
        );

        // Verify directory structure was created
        assert!(Path::new("./test_dir").exists());
        assert!(Path::new("./test_dir/nested").exists());
        assert!(Path::new("./test_dir/nested/deep").exists());
        assert!(Path::new(nested_path).exists());

        // Verify content
        let read_content =
            fs::read_to_string(nested_path).expect("Should read file");
        assert_eq!(read_content, content);

        // Cleanup
        drop(fs::remove_dir_all("./test_dir"));
    }

    #[test]
    fn mock_writer_specific_path_failure() {
        let failing_path = "./assets/openapi/should_fail.json";
        let mock_writer = MockFileWriter::new_failing_on_path(failing_path);
        let saver = OpenAPISaver::new(mock_writer.clone());
        let api = TestApi::openapi();

        // Should fail for specific path
        let result1 = saver.save_as(&api, "should_fail");
        assert!(result1.is_err());

        // Should succeed for different path
        let result2 = saver.save_as(&api, "should_succeed");
        result2.expect("Should succeed for different path");
        assert!(mock_writer.has_file("./assets/openapi/should_succeed.json"));
        assert!(!mock_writer.has_file(failing_path));
        assert_eq!(mock_writer.file_count(), 1);
    }

    #[test]
    fn mock_writer_concurrent_access() {
        use std::thread;

        let mock_writer = MockFileWriter::new();
        let _saver = OpenAPISaver::new(mock_writer.clone());
        let api = TestApi::openapi();

        // Simulate concurrent writes
        let handles: Vec<_> = (0_i32..5_i32)
            .map(|i| {
                let saver_clone = OpenAPISaver::new(mock_writer.clone());
                let api_clone = api.clone();
                thread::spawn(move || {
                    let result = saver_clone
                        .save_as(&api_clone, &format!("concurrent_{i}"));
                    assert!(
                        result.is_ok(),
                        "Concurrent write {i} should succeed"
                    );
                })
            })
            .collect();

        // Wait for all threads
        for handle in handles {
            handle.join().expect("Thread should complete successfully");
        }

        // Verify all files were written
        assert_eq!(mock_writer.file_count(), 5);
        for i in 0_i32..5_i32 {
            let path = format!("./assets/openapi/concurrent_{i}.json");
            assert!(mock_writer.has_file(&path), "File {i} should exist");
        }
    }

    #[test]
    fn openapi_saver_trait_integration() {
        let api = TestApi::openapi();
        let name = "integration_test_api";

        // This calls the trait implementation which uses DefaultFileWriter
        api.save_as(name).expect("Failed to save API spec");

        // Verify file exists
        let path =
            std::path::Path::new("./assets/openapi/integration_test_api.json");
        assert!(path.exists());

        // Cleanup
        std::fs::remove_file(path).expect("Failed to clean up test file");
    }
}
