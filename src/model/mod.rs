use json::{JsonValue, object};

pub struct Movie {
    name: String,
    description: String,
    image: String,
    movie_files: Vec<String>
}

const BASE_URL: &str = "http://movies.test/";

impl Movie {
    pub fn new(name: String, description: String, image: String, movie_files: Vec<String>) -> Self {
        Self {
            name, description, image, movie_files
        }
    }
    
    pub fn from_json(json: String) -> Option<Self> {
        let json = JsonValue::from(json);
        
        
        
        if let JsonValue::Object(obj) = json {
            let name = obj.get("name")?;
            let description = obj.get("description")?;
            let image = obj.get("image")?;
            let movie_files = obj.get("movie_files")?;
            if let JsonValue::Array(movies) = movie_files {
                return Some(Self {
                    name: name.to_string(), 
                    description: description.to_string(), 
                    image: image.to_string(), 
                    movie_files: movies.iter().map(|item| item.to_string()).collect()
                })
            }
        }
        
        None
    }
    
    pub fn json(self) -> JsonValue {
        object![
            name: self.name,
            description: self.description,
            image: self.image,
            movie_files: self.movie_files
        ]
    }
}

pub fn fetch() {
    
}