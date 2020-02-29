use markup;

markup::define! {
    Layout<'a>(page_title: &'a str, content: &'a str) {
        {markup::doctype()}
        html {
            head {
                title {
                    { page_title }
                }
                link [ rel="stylesheet", href="//cdnjs.cloudflare.com/ajax/libs/bulma/0.8.0/css/bulma.min.css"] {}
                link [ rel="stylesheet", href="//cdnjs.cloudflare.com/ajax/libs/highlight.js/9.18.1/styles/gruvbox-dark.min.css"] {}
                script [ src="//cdnjs.cloudflare.com/ajax/libs/highlight.js/9.18.1/highlight.min.js" ] {}
                script { "hljs.initHighlightingOnLoad();" }
            }
            body {
                div.section {
                    div.container {
                        div.content {
                            { markup::raw(content) }
                        }
                    }
                }
            }
        }
    }
}
