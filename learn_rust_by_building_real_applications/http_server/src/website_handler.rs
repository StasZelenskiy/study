use super::handler::Handler;
use super::http::{Request, Response, StatusCode, HttpMethod};
use std::fs;
use std::path::{PathBuf, Path};

pub struct WebsiteHandler {
    public_path: PathBuf,
}

impl WebsiteHandler {
    pub fn new(public_path: &str) -> Self {
        Self { public_path: fs::canonicalize(public_path).unwrap() }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = Path::join(&self.public_path, Self::remove_root(file_path));
        if path.starts_with(&self.public_path) {
            fs::read_to_string(path).ok()
        } else {
            println!("Directory Traversal Attack attempted: {}", file_path);
            None
        }
    }

    fn remove_root(file_path: &str) -> &str {
        for (i, c) in file_path.chars().enumerate() {
            if c != '\\' && c != '/' {
                return &file_path[i..];
            }
        }

        ""
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            HttpMethod::GET => {
                match request.path() {
                    "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                    "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                    path => match self.read_file(path) {
                        Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                        None => Response::new(StatusCode::NotFound, None),
                    },
                }
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
