use nu_plugin::{serve_plugin, EvaluatedCall, JsonSerializer, LabeledError, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, Span, SyntaxShape, Type, Value};

mod base;
use base::*;

struct Base;

impl Base {
    fn new() -> Self {
        Self {}
    }
}

impl Plugin for Base {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("from base")
            .usage("convert base")
            .category(Category::Strings)
            .optional("base", SyntaxShape::Int, "base of input")
            .named("table", SyntaxShape::String, "Source base table", Some('t'))
            .input_output_types(vec![
                (Type::Binary, Type::Binary),
                (Type::String, Type::Binary),
            ])
            .plugin_examples(vec![
                PluginExample {
                    example: r#""1am9llMhc" | from base 21 -t "123456789achlmnACHLMN""#.into(),
                    description: "convert base-51 to binary".into(),
                    result: Some(Value::Binary {
                        val: vec![0x04, 0x0A, 0xBA, 0xCA, 0x16],
                        span: Span::test_data(),
                    }),
                },
                PluginExample {
                    example: "0x[01 0A 17 10] | from base 21".into(),
                    description: "convert binary which already decoded as position number, to hex"
                        .into(),
                    result: Some(Value::Binary {
                        val: vec![0x37, 0x5A],
                        span: Span::test_data(),
                    }),
                },
            ])]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        match name {
            "from base" => match input {
                Value::Binary { val, .. } => {
                    if let Some(from) = call.opt::<usize>(0)? {
                        Ok(Value::Binary {
                            val: convert(&val, from, 16),
                            span: call.head,
                        })
                    } else {
                        Err(LabeledError {
                            label: "Binary input require base".into(),
                            msg: "add base".into(),
                            span: Some(call.head),
                        })
                    }
                }
                Value::String { val, span } => {
                    if let Some(table) = call.get_flag::<String>("table")? {
                        if let Some(from) = call.opt::<usize>(0)? {
                            if table.len() != from {
                                return Err(LabeledError {
                                    label: "Base does not match length of table".into(),
                                    msg: format!("Base is {}, but table is {}", from, table.len()),
                                    span: Some(call.head),
                                });
                            }
                        }
                        let binary = str_to_binary(&val, &table, span)?;
                        Ok(Value::Binary {
                            val: convert(&binary, table.len(), 16),
                            span: call.head,
                        })
                    } else {
                        Err(LabeledError {
                            label: "Encode table is require for string input".into(),
                            msg: "give a encode table with \"-t\"".into(),
                            span: Some(call.head),
                        })
                    }
                }
                _ => Err(LabeledError {
                    label: "Invalid input".into(),
                    msg: "This type of input is not expected".into(),
                    span: Some(call.head),
                }),
            },
            _ => Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "Unknown plugin command ".into(),
                span: Some(call.head),
            }),
        }
    }
}

fn main() {
    serve_plugin(&mut Base::new(), JsonSerializer)
}
