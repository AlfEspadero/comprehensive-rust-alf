use reqwest::Url;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::thread;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
	#[error("request error: {0}")]
	ReqwestError(#[from] reqwest::Error),
	#[error("bad http response: {0}")]
	BadResponse(String),
}

#[derive(Debug)]
struct CrawlCommand {
	url: Url,
	extract_links: bool,
}

fn visit_page(client: &Client, command: &CrawlCommand) -> Result<Vec<Url>, Error> {
	println!("Checking {:#}", command.url);
	let response = client.get(command.url.clone()).send()?;
	if !response.status().is_success() {
		return Err(Error::BadResponse(response.status().to_string()));
	}

	let mut link_urls = Vec::new();
	if !command.extract_links {
		return Ok(link_urls);
	}

	let base_url = response.url().to_owned();
	let body_text = response.text()?;
	let document = Html::parse_document(&body_text);

	let selector = Selector::parse("a").unwrap();
	let href_values = document
		.select(&selector)
		.filter_map(|element| element.value().attr("href"));
	for href in href_values {
		match base_url.join(href) {
			Ok(link_url) => {
				link_urls.push(link_url);
			}
			Err(err) => {
				println!("On {base_url:#}: ignored unparsable {href:?}: {err}");
			}
		}
	}
	Ok(link_urls)
}

fn main() {
	let client = Client::new();
	let start_url = Url::parse("https://www.google.org").unwrap();
	let crawl_command = CrawlCommand {
		url: start_url,
		extract_links: true,
	};

	// Create threads to check link validity but limit to 100 pages so we dont get blocked
	let mut page_count = 0;
	let mut handles = Vec::new();
	match visit_page(&client, &crawl_command) {
		Ok(links) => {
			for link in links {
				if page_count >= 100 {
					break;
				}
				page_count += 1;
				let client_clone = client.clone();
				let command = CrawlCommand {
					url: link,
					extract_links: false,
				};
				let handle = thread::spawn(move || match visit_page(&client_clone, &command) {
					Ok(_) => println!("Valid link: {:#}", command.url),
					Err(err) => println!("Broken link: {:#}, error: {err:#}", command.url),
				});
				handles.push(handle);
			}
			for handle in handles {
				handle.join().unwrap();
			}
		}
		Err(err) => println!("Could not extract links: {err:#}"),
	}
}
