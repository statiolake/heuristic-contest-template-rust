use itertools::{chain, Itertools as _};
use unicode_width::UnicodeWidthStr;

#[derive(Debug)]
pub struct Table {
    pub header: Vec<TableCell>,
    pub body: Vec<Vec<TableCell>>,
    pub footer: Vec<TableCell>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            header: vec![],
            body: vec![],
            footer: vec![],
        }
    }

    pub fn validate(&self) {
        let column_count = self.header.len();

        for row in &self.body {
            assert_eq!(row.len(), column_count, "Row has different column count");
        }

        assert_eq!(
            self.footer.len(),
            column_count,
            "Footer has different column count"
        );
    }

    pub fn print(&self) {
        self.validate();

        let column_content_widths = self.calculate_column_content_widths();

        self.print_row(&self.header, &column_content_widths);
        self.print_separator(&column_content_widths);
        for row in &self.body {
            self.print_row(row, &column_content_widths);
        }
        self.print_separator(&column_content_widths);
        self.print_row(&self.footer, &column_content_widths);
    }

    fn print_row(&self, row: &[TableCell], column_content_widths: &[usize]) {
        println!(
            "| {} |",
            row.iter()
                .zip(column_content_widths)
                .map(|(cell, &width)| cell.format(width))
                .format(" | ")
        );
    }

    fn print_separator(&self, column_content_widths: &[usize]) {
        println!(
            "|-{}-|",
            column_content_widths
                .iter()
                .map(|&width| "-".repeat(width))
                .format("-|-")
        );
    }

    fn calculate_column_content_widths(&self) -> Vec<usize> {
        (0..self.header.len())
            .map(|column_index| self.calculate_column_content_width_for(column_index))
            .collect()
    }

    fn calculate_column_content_width_for(&self, column_index: usize) -> usize {
        chain!(
            // Header
            Some(&self.header[column_index]),
            // Body
            self.body.iter().map(|row| &row[column_index]),
            // Footer
            Some(&self.footer[column_index]),
        )
        .map(|cell| UnicodeWidthStr::width(&*cell.content))
        .max()
        .unwrap_or(1)
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum Alignment {
    Left,
    Right,
}

#[derive(Debug)]
pub struct TableCell {
    pub content: String,
    pub alignment: Alignment,
}

impl TableCell {
    fn format(&self, width: usize) -> String {
        let padding = " ".repeat(width - UnicodeWidthStr::width(&*self.content));
        match self.alignment {
            Alignment::Left => format!("{}{}", self.content, padding),
            Alignment::Right => format!("{}{}", padding, self.content),
        }
    }
}
