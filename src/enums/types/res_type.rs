use crate::storage::record::record::Record;

#[derive(Debug)]
pub enum ResType {
    Success(String),
    View(View),
    Error(String),
}

#[derive(Debug)]
pub struct View {
    pub column_names: Vec<String>,
    pub values: Vec<Vec<String>>,
}

impl View {
    pub fn new(column_names: Vec<String>, records: Vec<Record>) -> Self {
        let mut values: Vec<Vec<String>> = Vec::with_capacity(column_names.len());
        for record in records {
            let mut columns: Vec<String> = Vec::new();
            record
                .columns()
                .iter()
                .for_each(|f| columns.push(f.to_string()));
            values.push(columns);
        }
        Self {
            column_names,
            values,
        }
    }

    pub fn serialize(&self) -> String {
        let mut output = String::new();

        // Column names
        output.push_str(&format!("{}\n", self.column_names.len()));
        for col in &self.column_names {
            output.push_str(&format!("{}:{}\n", col.len(), col));
        }

        // Rows
        output.push_str(&format!("{}\n", self.values.len()));
        for row in &self.values {
            for val in row {
                output.push_str(&format!("{}:{}\n", val.len(), val));
            }
        }

        output
    }

    pub fn deserialize(input: &str) -> Self {
        let mut lines = input.lines();

        // Read column names
        let num_cols: usize = lines.next().unwrap().parse().unwrap();
        let column_names: Vec<String> = (0..num_cols)
            .map(|_| {
                let line = lines.next().unwrap();
                let (len, val) = line.split_once(':').unwrap();
                assert_eq!(len.parse::<usize>().unwrap(), val.len());
                val.to_string()
            })
            .collect();

        // Read rows
        let num_rows: usize = lines.next().unwrap().parse().unwrap();
        let mut values = Vec::new();
        for _ in 0..num_rows {
            let mut row = Vec::new();
            for _ in 0..num_cols {
                let line = lines.next().unwrap();
                let (len, val) = line.split_once(':').unwrap();
                assert_eq!(len.parse::<usize>().unwrap(), val.len());
                row.push(val.to_string());
            }
            values.push(row);
        }

        View {
            column_names,
            values,
        }
    }

    pub fn display(&self) -> String {
        let mut result = String::new();

        // Compute column widths
        let mut col_widths: Vec<usize> = self.column_names.iter().map(|name| name.len()).collect();

        for row in &self.values {
            for (i, val) in row.iter().enumerate() {
                col_widths[i] = col_widths[i].max(val.len());
            }
        }

        // Helper to create a horizontal border like +------+------+
        let border = format!(
            "+{}+\n",
            col_widths
                .iter()
                .map(|w| "-".repeat(*w + 2))
                .collect::<Vec<_>>()
                .join("+")
        );

        // Add top border
        result.push_str(&border);

        // Add column names
        result.push_str(&format!(
            "| {} |\n",
            self.column_names
                .iter()
                .enumerate()
                .map(|(i, name)| format!("{:^width$}", name, width = col_widths[i]))
                .collect::<Vec<_>>()
                .join(" | ")
        ));

        // Add mid border
        result.push_str(&border);

        // Add each row
        for row in &self.values {
            result.push_str(&format!(
                "| {} |\n",
                row.iter()
                    .enumerate()
                    .map(|(i, val)| format!("{:^width$}", val, width = col_widths[i]))
                    .collect::<Vec<_>>()
                    .join(" | ")
            ));
        }

        // Add bottom border
        result.push_str(&border);
        result
    }
}
