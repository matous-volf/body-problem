use yew::{function_component, html, Html};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="py-4 flex flex-col gap-1 sm:flex-row sm:gap-0 justify-center items-center text-neutral-500 sm:divide-x divide-neutral-500">
            <span class="sm:pr-5">{"a simulation of the n-body problem"}</span>
            <span class="sm:pl-5"><a href="https://github.com/matous-volf/body-problem" target="_blank"><i class="fa-brands fa-github mr-2"></i><span class="underline">{"source code"}</span></a></span>
        </footer>
    }
}
