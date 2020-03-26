use error_chain::error_chain;

error_chain! {

    foreign_links {
        Io(::std::io::Error);
        JsonFmt(::serde_json::Error);
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
    }
}
