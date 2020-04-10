use reqwest::Url;
use std::fs::File;
use std::io::Write;

//TODO: Recover insted of panic
pub fn save_to_disk(url: &Url, content: &String) {
    let path = url_to_path(url);

    let mut file = match File::create(&path) {
        Err(err) => panic!("Couldn't create {}: {}", path, err),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(err) => panic!("Couldn't write to {}: {}", path, err),
        Ok(_) => (),
    };
}

fn url_to_path(url: &Url) -> String {
    let scheme_size = url.scheme().len() + 3; // 3 = "://".len()
    let url = url.as_str();

    let mut url = url.replace('/', "_").replace('.', "_");
    url.replace_range(0..scheme_size, ""); //Strip scheme
    let url = url.trim_end_matches('_'); //Remaining '/'

    return url.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_to_path() {
        let str = super::url_to_path(&Url::parse("https://lwn.net/Kernel/").unwrap());

        assert_eq!(str, "lwn_net_Kernel");
    }
}