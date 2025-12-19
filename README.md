# PricePeek.

# Price Tracker CLI (Rust) — CSV Price Comparison Tool

A simple Rust command-line application that helps track product prices across different shops/websites.  
It stores each entry (product + category + price + link + timestamp) in a local CSV file, lets you list entries, find the cheapest option, export filtered data, and delete entries.

## Problem Statement

When comparing prices across different shops/websites, it’s easy to lose links, forget the best price, or miss when a price was recorded.  
This tool automates the process by storing every price check with its URL and date/time in a CSV file and providing menu-driven actions to compare and export data.

## Features

- Add product prices with:
  - Product name
  - Category
  - Price
  - Product URL
  - Timestamp (auto-generated)
- List all stored prices
- Show the cheapest option (optionally filtered by category)
- Export data to a new CSV file (optionally filtered by category)
- Delete a stored entry from a numbered list

## Tech Stack

- Language: Rust
- Storage: CSV file (`prices.csv`)
- Dependencies:
  - `csv` for reading/writing CSV
  - `chrono` for timestamps
  - `anyhow` for simple error handling

## Project Structure

.
├── Cargo.toml
└── src
└── main.rs


## How It Works (Data Flow)

- The CSV file acts as the database (`prices.csv`).
- On first run, the tool creates the file and writes a header row:
  `product, category, price, url, timestamp`
- Each “Add” operation:
  - Prompts for product info
  - Captures current time (RFC3339)
  - Saves a new row
- “Cheapest”:
  - Reads all rows (or category-filtered rows)
  - Finds the minimum price
- “Export”:
  - Reads rows (optionally category-filtered)
  - Writes them to a new CSV file
- “Delete”:
  - Displays numbered items
  - Removes the selected row
  - Rewrites the CSV file

## Getting Started

### 1) Install Rust
Install Rust via rustup:
https://rustup.rs/

### 2) Run locally
Cargo Run


## Usage (Menu)

After running, you’ll see:

- `1) Add product price`
- `2) List all prices`
- `3) Show cheapest option`
- `4) Export data to CSV`
- `5) Delete a product`
- `6) Exit`

## CSV Format

File: `prices.csv`

Columns:
- `product` (string)
- `category` (string)
- `price` (number)
- `url` (string)
- `timestamp` (RFC3339 string)

## Example row:
AirPods,Electronics,199.99,https://example.com/product,2025-12-19T11:58:00Z



