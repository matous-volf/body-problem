module.exports = {
    content: [
        "./index.html",
        "./src/**/*.{rs,html,css}",
    ],
    theme: {
        fontFamily: {
            sans: ["'Ubuntu Sans'", "sans-serif"],
            mono: ["'Ubuntu Mono'", "monospace"],
        },
        extend: {
            screens: {
                "3xl": "1856px",
            },
        },
    },
    variants: {},
    plugins: [],
};
