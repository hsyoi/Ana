use std::path;
use std::process;

use super::*;

pub struct CppGxx;

impl CppGxx {
    pub fn new() -> CppGxx {
        CppGxx {}
    }
}

impl Compiler for CppGxx {
    fn compile(&self, source_file: &path::Path, executable_file: &path::Path) -> bool {
        let source_file =
            rename_with_new_extension(&source_file, "cpp").expect("Failed to rename source file");
        process::Command::new("g++")
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::piped())
            .arg(source_file.as_os_str())
            .arg("-o")
            .arg(executable_file.as_os_str())
            .arg("-O2")
            .arg("-fno-asm")
            .arg("-Wall")
            .arg("-lm")
            .arg("--static")
            .arg("--std=c++11")
            .spawn()
            .expect("Failed to run g++")
            .wait()
            .expect("Failed when waiting compiler to exit")
            .success()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    use tempfile;

    #[test]
    fn test_cpp_gxx_compile() {
        let work_dir = tempfile::tempdir().unwrap();
        let source_file = work_dir.path().join("cpp_compiler_test_pass.cpp");
        let executable_file = work_dir.path().join("cpp_compiler_test_pass.exe");
        fs::write(&source_file, "#include<iostream>\nint main() { return 0; }").unwrap();
        let compile_success = CppGxx::new().compile(&source_file, &executable_file);
        assert!(compile_success);
    }

    #[test]
    fn test_cpp_gxx_compile_failed() {
        let work_dir = tempfile::tempdir().unwrap();
        let source_file = work_dir.path().join("cpp_compiler_test_fail.cpp");
        let executable_file = work_dir.path().join("cpp_compiler_test_fail.exe");
        fs::write(&source_file, "#include<iostream>\nint main() { return 0 }").unwrap();
        let compile_success = CppGxx::new().compile(&source_file, &executable_file);
        assert!(!compile_success);
    }
}
