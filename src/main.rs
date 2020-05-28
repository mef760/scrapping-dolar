#[macro_use]
extern crate prettytable;
extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Class, Name };
use prettytable::Table;

fn main() {
    hacker_news("https://www.dolarhoy.com");
}

fn hacker_news(url: &str) {

    let resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    let document = Document::from_read(resp).unwrap();

    let mut table = Table::new();
    table.add_row(row![FwBbbc->"TIPO", FwBbbc->"COTIZACION"]);

    for node in document.find(Class("pill-coti")) {
        let name = node.find(Name("a")).next().unwrap().text();
       
        let values = node.find(Name("span")).map(|tag| tag.text().replace(" ", "").replace("\n",""))
        .collect::<Vec<_>>();

        let prices = format!("{}", values.join(" - "));
 
        // [FdBybl->] specifies row formatting
        // F (foreground) d (black text)
        // B (background) y (yellow text) l (left-align)
  
        table.add_row(row![name, prices]);
 
    }
    // print table to stdout
    table.printstd();
    println!("");
}