use starpls_bazel::builtin::Callable;
use starpls_bazel::builtin::Param;
use starpls_bazel::builtin::Type;
use starpls_bazel::builtin::Value;
use starpls_bazel::Builtins;
use starpls_syntax::TextRange;
use starpls_syntax::TextSize;

pub const CURSOR_MARKER: &str = "$0";

pub struct Fixture {
    pub contents: String,
    pub cursor_pos: TextSize,
    pub selected_ranges: Vec<TextRange>,
}

impl Fixture {
    pub fn parse(input: &str) -> Self {
        let offset = input.find(CURSOR_MARKER).unwrap();
        let mut contents = String::new();
        contents.push_str(&input[..offset]);
        contents.push_str(&input[offset + CURSOR_MARKER.len()..]);

        let selected_ranges = find_selected_ranges(&contents);

        Self {
            contents,
            cursor_pos: (offset as u32).into(),
            selected_ranges,
        }
    }
}

fn find_selected_ranges(contents: &str) -> Vec<TextRange> {
    let mut line_starts = vec![TextSize::new(0)];
    let mut ranges = Vec::new();
    for line in contents.split_inclusive('\n') {
        if let Some(start) = line.find("#^") {
            let remaining = &line[start + "#^".len()..];
            let additional_carets = remaining.chars().take_while(|c| c == &'^').count();
            if let Some(prev_line_start) = line_starts.get(line_starts.len() - 2) {
                let range_start = prev_line_start + TextSize::try_from(start).unwrap();
                let range_end =
                    range_start + TextSize::try_from("#^".len() + additional_carets).unwrap();
                ranges.push(TextRange::new(range_start, range_end))
            }
        }
        line_starts.push(line_starts.last().unwrap() + TextSize::of(line));
    }
    ranges
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FixtureType {
    pub name: String,
    pub fields: Vec<(String, String)>,
    pub methods: Vec<String>,
}

impl FixtureType {
    pub fn new(name: &str, fields: Vec<(&str, &str)>, methods: Vec<&str>) -> Self {
        FixtureType {
            name: name.into(),
            fields: fields
                .into_iter()
                .map(|(name, ty)| (name.into(), ty.into()))
                .collect(),
            methods: methods.into_iter().map(|method| method.into()).collect(),
        }
    }
}

pub fn make_test_builtins(
    functions: Vec<impl ToString>,
    globals: Vec<(String, String)>,
    types: Vec<FixtureType>,
) -> Builtins {
    Builtins {
        global: functions
            .into_iter()
            .map(|name| Value {
                name: name.to_string(),
                callable: Some(Callable {
                    param: vec![
                        Param {
                            name: "*args".to_string(),
                            is_star_arg: true,
                            ..Default::default()
                        },
                        Param {
                            name: "**kwargs".to_string(),
                            is_star_star_arg: true,
                            ..Default::default()
                        },
                    ],
                    return_type: "Unknown".to_string(),
                }),
                ..Default::default()
            })
            .chain(globals.into_iter().map(|(name, ty)| Value {
                name,
                r#type: ty,
                ..Default::default()
            }))
            .collect(),
        r#type: types
            .into_iter()
            .map(|ty| Type {
                name: ty.name,
                field: ty
                    .fields
                    .into_iter()
                    .map(|field| Value {
                        name: field.0,
                        r#type: field.1,
                        ..Default::default()
                    })
                    .chain(ty.methods.into_iter().map(|name| Value {
                        name,
                        callable: Some(Callable {
                            param: vec![
                                Param {
                                    name: "args".to_string(),
                                    is_star_arg: true,
                                    ..Default::default()
                                },
                                Param {
                                    name: "kwargs".to_string(),
                                    is_star_star_arg: true,
                                    ..Default::default()
                                },
                            ],
                            return_type: "Unknown".to_string(),
                        }),
                        ..Default::default()
                    }))
                    .collect(),
                ..Default::default()
            })
            .collect(),
    }
}
