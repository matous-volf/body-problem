use yew::hook;
use yew_hooks::use_effect_once;

//noinspection SpellCheckingInspection
const DOMAIN: &str = "body-problem.matousvolf.cz";
const WEBSITE_ID: &str = "705d6a0d-cff9-4bac-a953-8a89d6fc4fec";

#[hook]
pub(crate) fn use_umami_analytics() {
    use_effect_once(|| {
        let document = web_sys::window().unwrap().document().unwrap();
        let head = document.head().unwrap();

        let script = document.create_element("script").unwrap();
        script
            .set_attribute("src", "https://analytics.matousvolf.cz/script.js")
            .unwrap();
        script.set_attribute("data-website-id", WEBSITE_ID).unwrap();
        script.set_attribute("data-domains", DOMAIN).unwrap();
        script.set_attribute("defer", "").unwrap();
        head.append_child(&script).unwrap();

        || ()
    });
}
