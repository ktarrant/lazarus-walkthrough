mod type_chart;

fn main() {
    println!("# Type Matchups (Gen 6+)");
    println!();
    println!("This table is generated from the structured data in `src/type_chart.rs`.");
    println!("Run `cargo run > book/src/type-matchups.md` whenever the source data changes.");
    println!();
    print!("{}", type_chart::markdown_table());
}
