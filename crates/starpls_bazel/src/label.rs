use std::fmt;
use std::str::Chars;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RepoKind {
    Canonical,
    Apparent,
    Current,
}

#[derive(Clone, Debug)]
pub struct Label<'a> {
    source: &'a str,
    kind: RepoKind,
    is_relative: bool,
    repo_start: u32,
    repo_end: u32,
    package_start: u32,
    package_end: u32,
    target_start: u32,
    target_end: u32,
}

impl<'a> Label<'a> {
    pub fn parse(input: &'a str) -> ParseResult {
        Parser {
            chars: input.chars(),
            pos: 0,
            label: Label {
                source: input,
                kind: RepoKind::Current,
                is_relative: false,
                repo_start: 0,
                repo_end: 0,
                package_start: 0,
                package_end: 0,
                target_start: 0,
                target_end: 0,
            },
        }
        .parse()
    }

    pub fn kind(&self) -> RepoKind {
        self.kind.clone()
    }

    pub fn repo(&self) -> &str {
        self.slice(self.repo_start, self.repo_end)
    }

    pub fn package(&self) -> &str {
        self.slice(self.package_start, self.package_end)
    }

    pub fn target(&self) -> &str {
        self.slice(self.target_start, self.target_end)
    }

    pub fn is_relative(&self) -> bool {
        self.is_relative
    }

    pub fn slice(&self, start: u32, end: u32) -> &str {
        &self.source[start as usize..end as usize]
    }
}

#[derive(Clone, Debug)]
pub struct PartialParse<'a> {
    pub partial: Label<'a>,
    pub err: ParseError,
}

impl PartialEq for PartialParse<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.err == other.err
    }
}

impl Eq for PartialParse<'_> {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidRepo,
    InvalidPackage,
    InvalidTarget,
    EmptyPackage,
    EmptyTarget,
}

impl fmt::Display for PartialParse<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.err {
            ParseError::InvalidRepo => "invalid repo",
            ParseError::InvalidPackage => "invalid package",
            ParseError::InvalidTarget => "invalid target",
            ParseError::EmptyPackage => "empty package",
            ParseError::EmptyTarget => "empty target",
        })
    }
}

pub type ParseResult<'a> = Result<Label<'a>, PartialParse<'a>>;

struct Parser<'a, 'b> {
    chars: Chars<'b>,
    pos: u32,
    label: Label<'a>,
}

impl<'a, 'b> Parser<'a, 'b> {
    fn parse(mut self) -> ParseResult<'a> {
        match self.parse_full() {
            Ok(_) => Ok(self.label),
            Err(err) => Err(PartialParse {
                partial: self.label,
                err,
            }),
        }
    }

    fn parse_full(&mut self) -> Result<(), ParseError> {
        self.label.kind = self.parse_repo()?;
        let mut has_leading_slashes = false;
        match self.first() {
            Some('/') => {
                self.bump();
                if self.bump() != Some('/') {
                    return Err(ParseError::InvalidRepo);
                }
                has_leading_slashes = true;
            }
            None if self.label.repo_end > self.label.repo_start => {
                self.label.target_start = self.label.repo_start;
                self.label.target_end = self.label.repo_end;
                return Ok(());
            }
            None => return Err(ParseError::EmptyTarget),
            _ => {}
        }

        if self.chars.as_str().contains(':') {
            self.parse_package()?;
            assert_eq!(self.bump(), Some(':'));
        } else if self.pos == 0 {
        } else {
            self.parse_package()?;
            return if self.label.package_start == self.label.package_end {
                Err(ParseError::EmptyPackage)
            } else {
                self.label.target_start = self.label.package_start;
                self.label.target_end = self.label.package_end;
                Ok(())
            };
        }

        if self.label.package_start == self.label.package_end {
            self.label.is_relative = !has_leading_slashes;
        }

        self.parse_target()?;
        Ok(())
    }

    fn parse_repo(&mut self) -> Result<RepoKind, ParseError> {
        Ok(if let Some('@') = self.first() {
            self.bump();
            let kind = if let Some('@') = self.first() {
                self.bump();
                RepoKind::Canonical
            } else {
                RepoKind::Apparent
            };
            self.parse_repo_name()?;
            kind
        } else {
            RepoKind::Current
        })
    }

    fn parse_repo_name(&mut self) -> Result<(), ParseError> {
        self.label.repo_start = self.pos;
        while let Some(c) = self.first() {
            match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '_' | '.' | '-' => {
                    self.bump();
                }
                '~' if self.pos > self.label.repo_start => {
                    self.bump();
                }
                '/' => break,
                _ => return Err(ParseError::InvalidRepo),
            }
        }
        self.label.repo_end = self.pos;
        Ok(())
    }

    fn parse_package(&mut self) -> Result<(), ParseError> {
        let (start, end, has_target_only_chars) = match self.parse_package_or_target(true) {
            Ok(res) => res,
            Err(_) => return Err(ParseError::InvalidPackage),
        };
        if has_target_only_chars {
            return Err(ParseError::InvalidPackage);
        }
        self.label.package_start = start;
        self.label.package_end = end;
        Ok(())
    }

    fn parse_target(&mut self) -> Result<(), ParseError> {
        let (start, end, _) = match self.parse_package_or_target(false) {
            Ok(res) => res,
            Err(_) => return Err(ParseError::InvalidTarget),
        };
        if start == end {
            return Err(ParseError::EmptyTarget);
        }
        self.label.target_start = start;
        self.label.target_end = end;
        Ok(())
    }

    fn parse_package_or_target(&mut self, allow_colon: bool) -> Result<(u32, u32, bool), ()> {
        let start = self.pos;
        let mut has_target_only_chars = false;
        while let Some(c) = self.first() {
            match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '/' | '-' | '.' | '@' | '_' => {
                    self.bump();
                }
                '!' | '%' | '^' | '"' | '#' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ','
                | ';' | '<' | '=' | '>' | '?' | '[' | ']' | '{' | '|' | '}' | '~' => {
                    self.bump();
                    has_target_only_chars = true;
                }
                ':' if allow_colon => break,
                _ => return Err(()),
            }
        }
        Ok((start, self.pos, has_target_only_chars))
    }

    fn bump(&mut self) -> Option<char> {
        self.pos += 1;
        self.chars.next()
    }

    fn first(&mut self) -> Option<char> {
        self.chars.clone().next()
    }
}

#[cfg(test)]
mod tests {
    use super::{RepoKind::*, *};

    fn check(
        input: &str,
        kind: RepoKind,
        is_relative: bool,
        repo: &str,
        package: &str,
        target: &str,
    ) {
        let label = Label::parse(input).expect("expected successful parse");
        assert_eq!(label.kind(), kind);
        assert_eq!(label.is_relative(), is_relative);
        assert_eq!(label.repo(), repo);
        assert_eq!(label.package(), package);
        assert_eq!(label.target(), target);
    }

    fn check_err(input: &str, err: ParseError) {
        assert_eq!(
            Label::parse(input).expect_err("expected failed parse").err,
            err
        )
    }

    #[test]
    fn test_apparent_root_target_only() {
        check("@//:a", Apparent, false, "", "", "a")
    }

    #[test]
    fn test_apparent_package_target() {
        check("@//a:b", Apparent, false, "", "a", "b")
    }

    #[test]
    fn test_relative_target_only() {
        check(":a", Current, true, "", "", "a")
    }

    #[test]
    fn test_relative_target_only_no_colon() {
        check("a", Current, true, "", "", "a")
    }

    #[test]
    fn test_root_target() {
        check("//:a", Current, false, "", "", "a")
    }

    #[test]
    fn test_implicit_target() {
        check("//a", Current, false, "", "a", "a")
    }

    #[test]
    fn test_package_target() {
        check("//a:b", Current, false, "", "a", "b")
    }

    #[test]
    fn test_apparent_repo() {
        check("@a", Apparent, false, "a", "", "a")
    }

    #[test]
    fn test_apparent_repo_package() {
        check("@a//b", Apparent, false, "a", "b", "b")
    }

    #[test]
    fn test_apparent_repo_package_target() {
        check("@a//b:c", Apparent, false, "a", "b", "c")
    }

    #[test]
    fn test_apparent_repo_package_target_at_sign() {
        check("@a//@b:c", Apparent, false, "a", "@b", "c")
    }

    #[test]
    fn test_apparent_repo_package_target_dots() {
        check("@..//b:c", Apparent, false, "..", "b", "c")
    }

    #[test]
    fn test_apparent_repo_package_target_dashes() {
        check("@--//b:c", Apparent, false, "--", "b", "c")
    }

    #[test]
    fn test_full() {
        check(
            "//api_proto:api.gen.pb.go_checkshtest",
            Current,
            false,
            "",
            "api_proto",
            "api.gen.pb.go_checkshtest",
        )
    }

    #[test]
    fn test_full_repo() {
        check(
            "@go_sdk//:src/cmd/go/testdata/mod/rsc.io_!q!u!o!t!e_v1.5.2.txt",
            Apparent,
            false,
            "go_sdk",
            "",
            "src/cmd/go/testdata/mod/rsc.io_!q!u!o!t!e_v1.5.2.txt",
        )
    }

    #[test]
    fn test_target_nonstandard() {
        check("//:a][b", Current, false, "", "", "a][b");
    }

    #[test]
    fn test_repo_1() {
        check(
            "@rules_python~0.0.0~pip~name_dep//:_pkg",
            Apparent,
            false,
            "rules_python~0.0.0~pip~name_dep",
            "",
            "_pkg",
        )
    }

    #[test]
    fn test_repo_2() {
        check(
            "@rules_python~0.0.0~pip~name//:dep_pkg",
            Apparent,
            false,
            "rules_python~0.0.0~pip~name",
            "",
            "dep_pkg",
        )
    }

    #[test]
    fn test_canonical() {
        check(
            "@@rules_python~0.26.0~python~python_3_10_x86_64-unknown-linux-gnu//:python_runtimes",
            Canonical,
            false,
            "rules_python~0.26.0~python~python_3_10_x86_64-unknown-linux-gnu",
            "",
            "python_runtimes",
        )
    }

    #[test]
    fn test_empty() {
        check_err("", ParseError::EmptyTarget);
    }

    #[test]
    fn test_missing_target() {
        check_err("@//:", ParseError::EmptyTarget);
    }

    #[test]
    fn test_missing_slahes() {
        check_err("@a:b", ParseError::InvalidRepo);
    }

    #[test]
    fn test_missing_package() {
        check_err("@a//", ParseError::EmptyPackage);
    }
}
