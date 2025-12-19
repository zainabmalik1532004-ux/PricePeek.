use anyhow::{Context, Result};
use chrono::Utc;
use std::io::{self, Write};
use std::path::Path;

const HEADER: [&str; 5] = ["product", "category", "price", "url", "timestamp"];

#[derive(Debug, Clone)]
struct Row {
    product: String,
    category: String,
    price: f64,
    url: String,
    timestamp: String,
}

fn ensure_db(path: &str) -> Result<()> {
    if !Path::new(path).exists() {
        let mut wtr = csv::Writer::from_path(path)
            .with_context(|| format!("Create {}", path))?;
        wtr.write_record(HEADER)?;
        wtr.flush()?;
    }
    Ok(())
}

fn append_row(path: &str, r: &Row) -> Result<()> {
    ensure_db(path)?;
    // Append by reading existing rows and rewriting (simple and safe).
    let mut rows = read_rows(path)?;
    rows.push(r.clone());
    write_rows(path, &rows)?;
    Ok(())
}

fn read_rows(path: &str) -> Result<Vec<Row>> {
    ensure_db(path)?;
    let mut rdr = csv::Reader::from_path(path)?;
    let mut out = Vec::new();

    for rec in rdr.records() {
        let rec = rec?;
        // Support both old 4-column files and new 5-column files.
        if rec.len() >= 5 {
            let price: f64 = rec.get(2).unwrap_or("0").parse().unwrap_or(0.0);
            out.push(Row {
                product: rec.get(0).unwrap_or("").to_string(),
                category: rec.get(1).unwrap_or("").to_string(),
                price,
                url: rec.get(3).unwrap_or("").to_string(),
                timestamp: rec.get(4).unwrap_or("").to_string(),
            });
        } else {
            let price: f64 = rec.get(1).unwrap_or("0").parse().unwrap_or(0.0);
            out.push(Row {
                product: rec.get(0).unwrap_or("").to_string(),
                category: "".to_string(),
                price,
                url: rec.get(2).unwrap_or("").to_string(),
                timestamp: rec.get(3).unwrap_or("").to_string(),
            });
        }
    }
    Ok(out)
}

fn write_rows(path: &str, rows: &[Row]) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)?; // from_path truncates then writes [web:21]
    wtr.write_record(HEADER)?;
    for r in rows {
        wtr.write_record([
            r.product.as_str(),
            r.category.as_str(),
            &format!("{:.2}", r.price),
            r.url.as_str(),
            r.timestamp.as_str(),
        ])?;
    }
    wtr.flush()?;
    Ok(())
}

fn print_row(r: &Row) {
    println!("{} | {} | {:.2} | {} | {}", r.product, r.category, r.price, r.url, r.timestamp);
}

fn prompt_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}

fn main() -> Result<()> {
    let db = "prices.csv";
    ensure_db(db)?;

    loop {
        println!("\n== Price Tracker ==");
        println!("1) Add product price");
        println!("2) List all prices");
        println!("3) Show cheapest option");
        println!("4) Export data to CSV");
        println!("5) Delete a product");
        println!("6) Exit");

        let choice = prompt_input("Select an option: ")?;
        match choice.as_str() {
            "1" => {
                let product = prompt_input("Product name: ")?;
                let category = prompt_input("Category: ")?;
                let price_s = prompt_input("Price: ")?;
                let url = prompt_input("Product link (URL): ")?;
                let price: f64 = price_s.replace(',', ".").parse().context("Invalid price")?;
                let timestamp = Utc::now().to_rfc3339();
                let row = Row { product, category, price, url, timestamp };
                append_row(db, &row)?;
                println!("Saved.");
            }

            "2" => {
                let rows = read_rows(db)?;
                if rows.is_empty() {
                    println!("No entries.");
                } else {
                    for r in rows {
                        print_row(&r);
                    }
                }
            }

            "3" => {
                let rows = read_rows(db)?;
                if rows.is_empty() {
                    println!("No entries.");
                } else {
                    let cat = prompt_input("Category to search (leave empty for all): ")?;
                    let filtered: Vec<Row> = if cat.is_empty() {
                        rows
                    } else {
                        rows.into_iter().filter(|r| r.category.eq_ignore_ascii_case(&cat)).collect()
                    };
                    if filtered.is_empty() {
                        println!("No entries for that category.");
                    } else {
                        let best = filtered.into_iter().min_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(std::cmp::Ordering::Equal));
                        if let Some(b) = best {
                            println!("Cheapest option:");
                            print_row(&b);
                        }
                    }
                }
            }

            "4" => {
                let confirm = prompt_input("Export data to CSV? (y/N): ")?;
                if matches!(confirm.to_lowercase().as_str(), "y" | "yes") {
                    let out = prompt_input("Filename (default export.csv): ")?;
                    let out = if out.is_empty() { "export.csv" } else { &out };
                    let cat = prompt_input("Category to export (leave empty for all): ")?;
                    // write current rows to `out`
                    let rows = read_rows(db)?;
                    let rows: Vec<Row> = if cat.is_empty() {
                        rows
                    } else {
                        rows.into_iter().filter(|r| r.category.eq_ignore_ascii_case(&cat)).collect()
                    };
                    let mut wtr = csv::Writer::from_path(out).with_context(|| format!("Create {}", out))?;
                    wtr.write_record(HEADER)?;
                    for r in rows {
                        wtr.write_record([
                            r.product.as_str(),
                            r.category.as_str(),
                            &format!("{:.2}", r.price),
                            r.url.as_str(),
                            r.timestamp.as_str(),
                        ])?;
                    }
                    wtr.flush()?;
                    println!("Exported to {}", out);
                } else {
                    println!("Export canceled.");
                }
            }

            "5" => {
                // Delete a product by selecting from a numbered list (product | price)
                let mut rows = read_rows(db)?;
                if rows.is_empty() {
                    println!("No entries.");
                } else {
                    for (i, r) in rows.iter().enumerate() {
                        println!("{}: {} | {:.2}", i + 1, r.product, r.price);
                    }
                    let sel = prompt_input("Number to delete (or empty to cancel): ")?;
                    if sel.is_empty() {
                        println!("Canceled.");
                    } else {
                        let n: usize = match sel.parse() {
                            Ok(v) => v,
                            Err(_) => { println!("Invalid number."); continue; }
                        };
                        if n == 0 || n > rows.len() {
                            println!("Out of range.");
                            continue;
                        }
                        let idx = n - 1;
                        let choice = &rows[idx];
                        let confirm = prompt_input(&format!("Delete '{}' ({} )? (y/N): ", choice.product, choice.price))?;
                        if matches!(confirm.to_lowercase().as_str(), "y" | "yes") {
                            rows.remove(idx);
                            write_rows(db, &rows)?;
                            println!("Deleted.");
                        } else {
                            println!("Canceled.");
                        }
                    }
                }
            }

            "6" => {
                println!("Goodbye.");
                break;
            }

            _ => println!("Invalid option."),
        }
    }

    Ok(())
}
