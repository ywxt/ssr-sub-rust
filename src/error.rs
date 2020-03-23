use error_chain::error_chain;

error_chain! {

    errors {
        InvaildSsrUrl (url:String) {
             display("无效的SSR链接: '{}'", url)
        }
        InvaildSsrParam (params:String) {
             display("无效的SSR参数:{}", params)
        }
        InvaildSsrPath(path:String) {
            display("无效的SSR路径:{}", path)
        }
    }
}