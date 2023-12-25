use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use csv::ReaderBuilder;
use url::Url;

fn generate_test_code(websites: &[String], folder_path: &Path) {
    const TEMPLATE: &str = r#"describe('Compare Local and Production', () => {
        const pageName = '{page}';
    
        const testPage = (url) => {
    
          it(`should visually look the same for ${url}`, () => {
            Cypress.on('uncaught:exception', () => false);
            cy.visit(url);
            cy.document().its('readyState').should('eq', 'complete');
            cy.wait(5000);
            cy.compareSnapshot(pageName, 0.01);
            Cypress.on('uncaught:exception', () => true);
          });
        };
    
        testPage('{remoteUrl}');
        testPage('{localUrl}');
      });"#;

    for website in websites {
        let page_url = Url::parse(&website).unwrap_or_else(|_| {
            let mut url = Url::parse("http://localhost:8080").unwrap();
            url.set_path(&website);
            url
        });
        let page_name = page_url.path().trim_end_matches('/').trim_start_matches('/');

        let local_site = format!("http://localhost:8080{}", page_url.path());

        let code = TEMPLATE
            .replace("{page}", &page_name)
            .replace("{localUrl}", &local_site)
            .replace("{remoteUrl}", &website);

        let folder_name = page_name.trim_start_matches('/');
        let folder_path = folder_path.join(folder_name);
        fs::create_dir_all(&folder_path).expect("Failed to create folder");

        let path = folder_path.join("vcompare.cy.js");
        let mut file = File::create(path).expect("Failed to create file");
        file.write_all(code.as_bytes()).expect("Failed to write file");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 5 {
        println!("Usage: ./test_generator <csv_file> <column_number> <start_row> <end_row>");
        return;
    }

    let csv_file = &args[1];
    let column_number: usize = args[2].parse().unwrap_or(0) - 1; // Subtract 1 to convert to zero-based index
    let start_row: usize = args[3].parse().unwrap_or(0);
    let end_row: usize = args[4].parse().unwrap_or(0);

    let mut websites = Vec::new();

    let file = File::open(csv_file).expect("Failed to open file");
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    for _ in 0..start_row {
        let _ = reader.records().next();
    }

    for (i, result) in reader.records().enumerate() {
        if i >= end_row - start_row {
            break;
        }

        let record = result.expect("Failed to read record");
        if let Some(website) = record.get(column_number) {
            if website != "なし(TOPへ)" && website != "なし" && website != "なし（TOPへ）" {
                websites.push(website.to_owned());
            }
        }
    }

    let folder_path = Path::new("cypress");
    fs::create_dir_all(&folder_path).expect("Failed to create cypress folder");

    generate_test_code(&websites, &folder_path);

    println!("Test code generation completed.");
}
