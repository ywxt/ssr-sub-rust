use error_chain::error_chain;

error_chain! {

    foreign_links {
        Io(::std::io::Error);
        JsonFmt(::serde_json::Error);
        Http(::reqwest::Error);
    }

    errors {
        InvalidSsrUrl (url:String) {
             display("无效的SSR链接: '{}'", url)
        }
        InvalidSsrParam (params:String) {
             display("无效的SSR参数:{}", params)
        }
        InvalidSsrPath(path:String) {
            display("无效的SSR路径:{}", path)
        }
        InvalidSsrSubscription(content:String) {
            display("无效的SSR订阅:{}", content)
        }
    }
}
