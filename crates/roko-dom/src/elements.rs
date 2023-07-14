/// This module contains functions for creating HTML elements. These functions are used by the
/// `roko_macro` crate to create the HTML elements that are rendered to the DOM.
use roko_html::{Attrs, Children, Html};

macro_rules! elements {
    ($($name:ident),+) => {
        $(
            #[inline(always)]
            pub fn $name<Msg>(
                id: Option<String>,
                attrs: Attrs<Msg>,
                children: Children<Msg>,
            ) -> Html<Msg> {
                Html::node(stringify!($name), id, attrs, children)
            }
        )+
    };
    () => {};
}

elements! {
    p, div, span, main, a, ul, li, img, h1, h2, h3, h4, h5, h6, button, input, form,
    textarea, label, pre, code, select, option, table, thead, tbody, tr, td, th, i, b, strong,
    em, small, hr, nav, footer, article, aside, details, summary, fieldset, legend,
    audio, video, canvas, figure, figcaption, time, datalist, keygen, output, progress, meter,
    rt, rp, samp, sub, sup, mark, wbr, bdi, bdo, cite, q, dfn, abbr, address, del, ins, menu,
    header, section
}
