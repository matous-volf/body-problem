use yew::hook;
use yew_hooks::use_effect_once;

//noinspection SpellCheckingInspection
const GOOGLE_ANALYTICS_TAG_ID: &str = "G-CNGHF1MBDT";

#[hook]
pub(crate) fn use_google_analytics() {
    use_effect_once(|| {
        let document = web_sys::window().unwrap().document().unwrap();
        let head = document.head().unwrap();

        let script = document.create_element("script").unwrap();
        script.set_attribute("async", "").unwrap();
        script.set_attribute(
            "src", 
            format!("https://www.googletagmanager.com/gtag/js?id={GOOGLE_ANALYTICS_TAG_ID}").as_str()
        ).unwrap();
        head.append_child(&script).unwrap();

        let inline_script = document.create_element("script").unwrap();
        inline_script.set_inner_html(format!("
            window.dataLayer = window.dataLayer || [];
            function gtag(){{dataLayer.push(arguments);}}
            gtag('js', new Date());
            gtag('config', '{GOOGLE_ANALYTICS_TAG_ID}');
        ").as_str());
        head.append_child(&inline_script).unwrap();
        
        || ()
    });
}
