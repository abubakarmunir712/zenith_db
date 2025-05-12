use std::sync::Arc;

use serde::de::value;

use crate::{
    enums::{
        commands::{cmd::SqlCommand, cmd_obj::ColumnDef},
        errors::parser_errors::ParserError,
        types::{datatypes::DataType, res_type::ResType},
    },
    executor::executor::Executor,
};

pub struct Parser;

impl Parser {
    pub fn parse_query(query: &str, db_name: &str, executor: Arc<Executor>) -> ResType {
        let tokens: Vec<&str> = query.split(" ").collect();
        if tokens.len() < 2 {
            return ResType::Error(ParserError::InvalidSyntax.message().to_string());
        } else if tokens[0].trim().to_lowercase() == "create_db" && tokens.len() == 2 {
            let cmd = SqlCommand::CreateDB {
                db_name: tokens[1].to_string(),
            };
            return executor.execute(cmd);
        } else if tokens[0].trim().to_lowercase() == "create" {
            let table_name = tokens[1];
            if tokens.len() < 3 {
                return ResType::Error(ParserError::ColumnNamesRequired.message().to_string());
            }
            let columns = tokens[2..tokens.len()].join(" ");
            let columns = columns.trim();
            if columns.starts_with("(") && columns.ends_with(")") {
                let columns = &columns[1..columns.len() - 1];
                let mut cols: Vec<ColumnDef> = Vec::new();
                let cols_info: Vec<&str> = columns.split(",").collect();
                for col in cols_info {
                    let col_info: Vec<&str> = col.split(":").collect();
                    if col_info.len() > 2 {
                        return ResType::Error(
                            ParserError::InvalidColumnName.message().to_string(),
                        );
                    } else if col_info.len() < 2 {
                        return ResType::Error(ParserError::DataTypeRequired.message().to_string());
                    } else {
                        let dt_v: Vec<&str> = col_info[1].split("[").collect();
                        if dt_v.len() > 2 {
                            return ResType::Error(
                                ParserError::InvalidSyntax.message().to_string(),
                            );
                        }
                        let datatype = DataType::from_string(dt_v[0].trim());
                        let c_name = col_info[0].trim();
                        if c_name == "" {
                            return ResType::Error(
                                ParserError::ColumnNamesRequired.message().to_string(),
                            );
                        }
                        match datatype {
                            Ok(dt) => {
                                let mut d_size = 0;
                                match dt {
                                    DataType::CHAR | DataType::VARCHAR => {
                                        if dt_v.len() != 2 {
                                            return ResType::Error(
                                                ParserError::LenghtRequired.message().to_string(),
                                            );
                                        }
                                        let dt_v = dt_v[1].to_string();
                                        let dt_v = &dt_v[0..dt_v.len() - 1];
                                        let dt_v = dt_v.trim();

                                        let res_parse: Result<u32, _> = dt_v.parse();
                                        if res_parse.is_err() {
                                            return ResType::Error(
                                                ParserError::InvalidSyntax.message().to_string(),
                                            );
                                        }
                                        d_size = res_parse.unwrap();
                                    }
                                    _ => {
                                        if dt_v.len() != 1 {
                                            return ResType::Error(
                                                ParserError::InvalidSyntax.message().to_string(),
                                            );
                                        }
                                        d_size = dt.size();
                                    }
                                }
                                let col_e = ColumnDef {
                                    name: c_name.to_string(),
                                    datatype: dt,
                                    size: Some(d_size),
                                };
                                cols.push(col_e);
                            }
                            Err(e) => return ResType::Error(e),
                        }
                    }
                }
                let cmd = SqlCommand::CreateTable {
                    db_name: db_name.to_string(),
                    table: table_name.to_string(),
                    columns: cols,
                };
                return executor.execute(cmd);
            }
        } else if tokens[0].trim().to_lowercase() == "delete_db" && tokens.len() == 2 {
            if tokens[1].trim() == db_name {
                return ResType::Error(ParserError::DBInUse.message().to_string());
            }
            let cmd = SqlCommand::DeleteDB {
                db_name: tokens[1].to_string(),
            };
            return executor.execute(cmd);
        } else if tokens[0].trim().to_lowercase() == "delete" && tokens.len() == 2 {
            if tokens[1].trim() == db_name {
                return ResType::Error(ParserError::DBInUse.message().to_string());
            }
            let cmd = SqlCommand::DeleteTable {
                db_name: db_name.to_string(),
                table: tokens[1].to_string(),
            };
            return executor.execute(cmd);
        } else if tokens[0].trim().to_lowercase() == "make_null" && tokens.len() == 3 {
            let table_name = tokens[1];
            let col_name = tokens[2];
            let cmd = SqlCommand::MakeNull {
                db_name: db_name.to_string(),
                table: tokens[1].to_string(),
                column: col_name.to_string(),
            };
            return executor.execute(cmd);
        } else if tokens[0].trim().to_lowercase() == "make_primary" && tokens.len() == 3 {
            let table_name = tokens[1];
            let col_name = tokens[2];
            let cmd = SqlCommand::MakePrimary {
                db_name: db_name.to_string(),
                table: tokens[1].to_string(),
                column: col_name.to_string(),
            };
            return executor.execute(cmd);
        } else if tokens[0].trim().to_lowercase() == "insert" && tokens.len() >= 3 {
            let table_name = tokens[1];
            let res = parse_columns_values(&tokens[2..tokens.len()].join(" "));
            match res {
                Err(e) => {
                    return ResType::Error(e);
                }
                Ok(val) => {
                    let cmd = SqlCommand::Insert {
                        db_name: db_name.to_string(),
                        table: table_name.to_string(),
                        columns: val.0,
                        values: val.1,
                    };
                    return executor.execute(cmd);
                }
            }
        } else if tokens[0].trim().to_lowercase() == "select_all" && tokens.len() == 2 {
            let cmd = SqlCommand::Select {
                db_name: db_name.to_string(),
                table: tokens[1].to_string(),
                filter: None,
                all: true,
            };
            return executor.execute(cmd);
        } else if tokens[0].trim().to_lowercase() == "select" && tokens.len() >= 4 {
            let table = tokens[1].to_string();
            let column = tokens[2].to_string();
            let value = tokens[3..tokens.len()].join(" ");
            let value = value.trim();
            let cmd = SqlCommand::SelectByValue {
                db_name: db_name.to_string(),
                table,
                col: column,
                value: value.to_string(),
            };

            return executor.execute(cmd);
        } else if tokens[0].trim().to_lowercase() == "delete" && tokens.len() >= 4 {
            let table = tokens[1].to_string();
            let column = tokens[2].to_string();
            let value = tokens[3..tokens.len()].join(" ");
            let value = value.trim();
            let value = value.to_string();
            let cmd = SqlCommand::DeleteRecord {
                db_name: db_name.to_string(),
                table,
                col: column,
                value,
            };

            return executor.execute(cmd);
        } else {
            return ResType::Error(ParserError::InvalidSyntax.message().to_string());
        }

        return ResType::Error(ParserError::InvalidSyntax.message().to_string());
    }
}

fn parse_columns_values(input: &str) -> Result<(Vec<String>, Vec<String>), String> {
    // Trim whitespace and check basic structure
    let input = input.trim();
    if !input.starts_with('(') || !input.contains('{') || !input.ends_with('}') {
        println!("{input}");
        return Err("Invalid input format: must start with '(' and end with '}'".to_string());
    }

    // Split into columns and values parts
    let parts: Vec<&str> = input.split('{').collect();
    if parts.len() != 2 {
        return Err(
            "Invalid input: must contain exactly one '{' separating columns and values".to_string(),
        );
    }

    // Parse columns (inside parentheses)
    let cols_part = parts[0].trim();
    if !cols_part.starts_with('(') || !cols_part.ends_with(')') {
        return Err("Invalid columns format: must be enclosed in parentheses".to_string());
    }
    let cols_str = &cols_part[1..cols_part.len() - 1]; // Remove '(' and ')'
    let columns: Vec<String> = cols_str
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    if columns.is_empty() {
        return Err("No valid columns found".to_string());
    }

    // Parse values (inside curly braces and double quotes)
    let vals_part = parts[1].trim();
    if !vals_part.ends_with('}') {
        return Err("Invalid values format: must end with '}'".to_string());
    }
    let vals_str = &vals_part[..vals_part.len() - 1]; // Remove '}'
    let mut values: Vec<String> = Vec::new();
    let mut chars = vals_str.chars().enumerate();
    let mut current_val = String::new();
    let mut in_quotes = false;
    let mut escaped = false;

    while let Some((i, c)) = chars.next() {
        match c {
            '"' if !escaped => {
                in_quotes = !in_quotes;
                if !in_quotes {
                    values.push(current_val.trim().to_string());
                    current_val.clear();
                }
            }
            '\\' if in_quotes => {
                escaped = true;
            }
            c if in_quotes && escaped => {
                current_val.push(c);
                escaped = false;
            }
            c if in_quotes => {
                current_val.push(c);
            }
            ',' if !in_quotes => {
                // Ignore commas outside quotes
            }
            c if !in_quotes && !c.is_whitespace() => {
                return Err(format!("Unexpected character '{}' at position {}", c, i));
            }
            _ => {}
        }
    }

    if in_quotes {
        return Err("Unclosed double quote in values".to_string());
    }
    if values.is_empty() {
        return Err("No valid values found".to_string());
    }

    // Ensure the number of columns matches the number of values
    if columns.len() != values.len() {
        return Err(format!(
            "Mismatch: {} columns but {} values",
            columns.len(),
            values.len()
        ));
    }

    Ok((columns, values))
}
