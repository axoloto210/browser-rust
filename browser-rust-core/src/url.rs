use alloc::{
    string::{String, ToString},
    vec::Vec,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpart: String,
}

impl Url {
    pub fn new(url: String) -> Self {
        Self {
            //SelfはImplで指定している構造体の型
            url,
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        }
    }

    //&mut self でオブジェクトのプロパティを変更可能に
    pub fn parse(&mut self) -> Result<Self, String> {
        if !self.is_http() {
            return Err("Only http scheme allowed.".to_string());
        }

        self.host = self.extract_host();
        self.port = self.extract_port();
        self.path = self.extract_path();
        self.searchpart = self.extract_searchpart();

        Ok(self.clone())
    }

    fn is_http(&mut self) -> bool {
        return self.url.contains("http://");
    }

    fn extract_host(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        //ポート番号の有無を判定
        //Some型は値が存在することを明示する型。コンパイラが型チェックを行うので安全。
        if let Some(index) = url_parts[0].find(':') {
            //':'よりも前の文字を結合して返す。[..index]はRustのスライス構文で、文字列の部分参照（スライス）を作成。
            //'foo:8000'であれば、0番目からindex-1 番目のスライス'foo'を取得。マルチバイト文字は意図通りに分割できない（バイト単位）ため注意。
            url_parts[0][..index].to_string()
        } else {
            url_parts[0].to_string()
        }
    }

    fn extract_port(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if let Some(index) = url_parts[0].find(':') {
            url_parts[0][index + 1..].to_string()
        } else {
            "80".to_string()
        }
    }

    fn extract_path(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if url_parts.len() < 2 {
            return "".to_string();
        }

        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();
        path_and_searchpart[0].to_string()
    }

    fn extract_searchpart(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if url_parts.len() < 2 {
            return "".to_string();
        }

        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();
        if path_and_searchpart.len() < 2 {
            "".to_string()
        } else {
            path_and_searchpart[1].to_string()
        }
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub fn port(&self) -> String {
        self.port.clone()
    }
    pub fn path(&self) -> String {
        self.path.clone()
    }
    pub fn searchpart(&self) -> String {
        self.searchpart.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_host() {
        let url = "http://example.com".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_host_port() {
        let url = "http://example.com:8000".to_string();
        let expected: Result<Url, String> = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8000".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse())
    }

    #[test]
    fn test_url_port_path() {
        let url = "http://example.com:8000/index.html".to_string();
        let expected: Result<Url, String> = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8000".to_string(),
            path: "index.html".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse())
    }

    #[test]
    fn test_url_host_path() {
        let url = "http://example.com/index.html".to_string();
        let expected: Result<Url, String> = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "index.html".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse())
    }

    #[test]
    fn test_url_host_port_path_searchquery() {
        let url = "http://example.com:8000/index.html?foo=123&bar=baz".to_string();
        let expected: Result<Url, String> = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8000".to_string(),
            path: "index.html".to_string(),
            searchpart: "foo=123&bar=baz".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse())
    }

    #[test]
    fn test_no_scheme() {
        let url = "example.com".to_string();
        let expected = Err("Only http scheme allowed.".to_string());
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_unsupported_scheme(){
        let url = "https://example.com:8000/index.html".to_string();
        let expected = Err("Only http scheme allowed.".to_string());
        assert_eq!(expected, Url::new(url).parse())
    }
}
