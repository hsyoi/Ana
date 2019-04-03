use std::fs;
use std::io;
use std::path;
use std::process;
use std::str;

fn diff(output: &[u8], answer: &[u8]) -> bool {
    let output = match str::from_utf8(&output) {
        Ok(res) => res,
        _ => return false,
    };
    let answer = match str::from_utf8(&answer) {
        Ok(res) => res,
        _ => return false,
    };
    !output
        .trim_end_matches('\n')
        .split_terminator('\n')
        .zip(answer.trim_end_matches('\n').split_terminator('\n'))
        .all(|(output, answer)| output.trim_end_matches(' ') == answer.trim_end_matches(' '))
}

pub fn check(output_file: &path::Path, answer_file: &path::Path) -> io::Result<bool> {
    Ok(!diff(
        fs::read(&output_file)?.as_slice(),
        fs::read(&answer_file)?.as_slice(),
    ))
}

pub fn check_with_spj(
    input_file: &path::Path,
    output_file: &path::Path,
    answer_file: &path::Path,
    spj: &path::Path,
) -> io::Result<bool> {
    Ok(process::Command::new(spj)
        .arg(input_file)
        .arg(answer_file)
        .arg(output_file)
        .status()?
        .success())
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile;

    #[test]
    fn test_diff_complete_eq() {
        assert!(!diff(b"hello world", b"hello world"));
    }

    #[test]
    fn test_diff_with_empty_line_at_eof() {
        assert!(!diff(b"hello world\n", b"hello world"));
        assert!(!diff(b"hello world", b"hello world\n"));
    }

    #[test]
    fn test_diff_with_space_at_eol() {
        assert!(!diff(b"hello world ", b"hello world"));
        assert!(!diff(b"hello world", b"hello world "));
    }

    #[test]
    fn test_diff_with_both_empty_line_at_eof_and_space_at_eol() {
        assert!(!diff(b"hello world \n", b"hello world"));
        assert!(!diff(b"hello world", b"hello world \n"));
        assert!(!diff(b"hello world\n", b"hello world "));
        assert!(!diff(b"hello world ", b"hello world\n"));
        assert!(!diff(b"hello world \n", b"hello world\n"));
        assert!(!diff(b"hello world\n", b"hello world \n"));
        assert!(!diff(b"hello world\n ", b"hello world\n"));
        assert!(!diff(b"hello world\n", b"hello world\n "));
    }

    #[test]
    fn test_check() -> io::Result<()> {
        let work_dir = tempfile::tempdir()?;
        let file0 = work_dir.path().join("test_check_without_spj.0");
        let file1 = work_dir.path().join("test_check_without_spj.1");
        let file2 = work_dir.path().join("test_check_without_spj.2");
        fs::write(&file0, "hello world")?;
        fs::write(&file1, "hello world")?;
        fs::write(&file2, "hello_world")?;
        assert!(check(&file0, &file1)?);
        assert!(!check(&file0, &file2)?);
        Ok(())
    }

    // TODO: Add Spj test
    // #[test]
    // fn test_check_with_spj() -> io::Result<()> {
    //     unimplemented!()
    // }
}
