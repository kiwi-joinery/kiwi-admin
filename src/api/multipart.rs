use crate::api::RequestBody;
use headers::ContentType;
use mime::Mime;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::iter;
use yew::format::Binary;

pub struct Multipart {
    boundary: String,
    text_fields: Vec<(String, String)>,
    files: Vec<MultipartFile>,
}

pub struct MultipartFile {
    data: Vec<u8>,
    name: String,
    filename: Option<String>,
    mime: Option<Mime>,
}

impl MultipartFile {
    pub fn new(name: &str, data: Vec<u8>, filename: Option<String>) -> Self {
        Self {
            data,
            name: name.to_string(),
            filename,
            mime: None,
        }
    }
}

impl Multipart {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let boundary: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(65)
            .collect();
        Self {
            boundary,
            text_fields: Vec::new(),
            files: Vec::new(),
        }
    }

    pub fn add_text(&mut self, name: &str, value: String) {
        self.text_fields.push((name.to_string(), value));
    }

    pub fn add_file(&mut self, file: MultipartFile) {
        self.files.push(file);
    }
}

impl RequestBody for Multipart {
    fn content_type(&self) -> Option<ContentType> {
        let mime: Mime = format!("multipart/form-data; boundary={}", self.boundary)
            .parse()
            .unwrap();
        Some(ContentType::from(mime))
    }
}

impl Into<Binary> for Multipart {
    fn into(self) -> Binary {
        let mut data = Vec::new();

        /* The encapsulation boundary is defined as a line consisting entirely of two hyphen
        characters ("-", decimal code 45) followed by the boundary parameter value from the
        Content-Type header field. The boundary must be followed immediately either by another
        CRLF and the header fields for the next part, or by two CRLFs, in which case there are no
        header fields for the next part (and it is therefore assumed to be of Content-Type text/plain). */
        let encapsulation_boundary = format! {"--{}\r\n", self.boundary};

        for (name, value) in self.text_fields {
            data.extend_from_slice(encapsulation_boundary.as_bytes());
            data.append(
                &mut format!("Content-Disposition:form-data; name=\"{}\"\r\n\r\n", name)
                    .into_bytes(),
            );
            data.append(&mut value.into_bytes());
            data.extend_from_slice("\r\n".as_bytes());
        }

        for mut file in self.files {
            data.extend_from_slice(encapsulation_boundary.as_bytes());
            let mut s = format!("Content-Disposition:form-data; name=\"{}\";", file.name);
            match file.filename {
                None => {}
                Some(f) => s.push_str(format!(" filename=\"{}\";", f).as_str()),
            }
            s.push_str("\r\n");
            data.append(&mut s.into_bytes());
            data.append(
                &mut format!(
                    "Content-Type: {}\r\n\r\n",
                    file.mime.unwrap_or(mime::APPLICATION_OCTET_STREAM)
                )
                .into_bytes(),
            );
            data.append(&mut file.data);
            data.extend_from_slice("\r\n".as_bytes());
        }

        /* The encapsulation boundary following the last body part is a distinguished delimiter that
        indicates that no further body parts will follow. Such a delimiter is identical to the previous
        delimiters, with the addition of two more hyphens at the end of the line */
        data.append(&mut format!("--{}--\r\n", self.boundary).into_bytes());
        Ok(data)
    }
}
