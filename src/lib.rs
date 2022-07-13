use gloo_timers::callback::Timeout;
use js_sys::Math;
use wasm_bindgen::prelude::*;
use yew::html::Scope;
use yew::prelude::*;

const BASE: &str = "https://jocketf.se/tea";

const IMAGES: &[&'static str] = &[
    "2y2owhnB8aK.jpg",
    "2zEVTpNRRDU.jpg",
    "3YGkXAS7YES.jpg",
    "4bPkFoLwVcC.jpg",
    "6m4EVv9WfyX.jpg",
    "7kStWc2VtF4.jpg",
    "8aKUW95x81U.jpg",
    "8KVZHkb7nwT.jpg",
    "9h13ETSKxnT.jpg",
    "9u9oWRfzHBW.jpg",
    "ADjJP9XoG2j.jpg",
    "AMnfP8wwevH.jpg",
    "aPEGfbQf63f.jpg",
    "B1DctMaxJ9E.jpg",
    "B5wbmZZexzg.jpg",
    "bAM5vY3qsyF.jpg",
    "bgzE197Drja.jpg",
    "Brri3QjR6ji.jpg",
    "bSp21VY52xk.jpg",
    "bvji4RrxpTh.jpg",
    "ByEALubk73C.jpg",
    "C8u9tRvy22Z.jpg",
    "cQP7eac9uGy.jpg",
    "d2oc9pjfAiz.jpg",
    "D4d5deySFo8.jpg",
    "D5fkomyYXdv.jpg",
    "DHD9wtGUKKn.jpg",
    "djX8Cc7yiLD.jpg",
    "dMKt2CbPEmU.jpg",
    "dNAiog7JfHk.jpg",
    "dzV4Zqsqqve.jpg",
    "e2PQEY4A3yg.jpg",
    "Eb2UEj4Cdkx.jpg",
    "ENnGYEVTk7J.jpg",
    "eyduVMXorjU.jpg",
    "f6A4fG4dFbC.jpg",
    "Fe8tMPcnXA3.jpg",
    "FkRUVU7uNht.jpg",
    "fLKUsRCWpxT.jpg",
    "FP9QJFmQDyP.jpg",
    "fZczGCmyg7N.jpg",
    "FzXaYDgcRtZ.jpg",
    "G2FSD7URcRr.jpg",
    "g2wECkWGSRu.jpg",
    "gBv6z5WLuDv.jpg",
    "GjPqYUzMzdN.jpg",
    "goE5hcYZF4a.jpg",
    "gsAqAVHFuiD.jpg",
    "haDsGfvgmYG.jpg",
    "hBEioPWGZxx.jpg",
    "heBwERTF5EX.jpg",
    "HQzfLuZ8Cq5.jpg",
    "hWqos1M38Zo.jpg",
    "HyFmzaTmPwP.jpg",
    "isQ2rQMCvdS.jpg",
    "iyWVsVpLDfS.jpg",
    "J5B5WrHqFBn.jpg",
    "jFKiJ8tbahK.jpg",
    "jFU7asLtECK.jpg",
    "JGFxRb9j6R4.jpg",
    "jiv2FkychyU.jpg",
    "JrTM1SrhfXX.jpg",
    "JsQ8K5JRBPF.jpg",
    "Kj1AfScW7Fs.jpg",
    "KKLSWiFwU87.jpg",
    "KSwhRqbuN9u.jpg",
    "Lj5rK7moA5F.jpg",
    "LoaWufxznkv.jpg",
    "LtxjY4fx2Hb.jpg",
    "Lx5LhCYt5YG.jpg",
    "MHCghbJMqMB.jpg",
    "MQ4VLoovgYb.jpg",
    "MWAD2YaDsYm.jpg",
    "NjykgXBEqwb.jpg",
    "NmM9LSe2z9D.jpg",
    "NqsnY1aZZnb.jpg",
    "PGHFXt6yJfy.jpg",
    "PpWaaWiHT5C.jpg",
    "Py22FvuuHT4.jpg",
    "R2RkjR6YJKH.jpg",
    "RhGYQSRqn3J.jpg",
    "RJLqgDznEod.jpg",
    "SEMEvNbPTAe.jpg",
    "TLF4cEZx55F.jpg",
    "Uc3PmDn5z1E.jpg",
    "UR1J65oxAdD.jpg",
    "Us7RuRHiyxM.jpg",
    "WimoJjoE9aG.jpg",
    "WipLwxte5kG.jpg",
    "X271NuMKrx9.jpg",
    "XgtR4EKWHZS.jpg",
    "XsSyT4YwbPc.jpg",
    "Y2jMAq5ehct.jpg",
    "YhhQ9UcgRJ9.jpg",
    "YK5Nwn2r1pQ.jpg",
    "ZMPMXriZiP8.jpg",
    "ZVKKJLU1Y4b.jpg",
];

const STYLE: &str = r#"
    body {
        overflow: hidden;
        background-color: black;
        color: white;
        padding: 0;
        margin: 0;
    }

    img {
        position: absolute;
        transition: opacity 0.5s linear;
        object-fit: contain;
        height: 100vh;
        width: 100vw;
    }

    img.current {
        opacity: 255;
        z-index: 1;
    }

    img.standby {
        opacity: 0;
        z-index: 0;
    }
"#;

#[derive(Clone)]
enum Msg {
    Preload,
    Switch,
}

struct Slider {
    current: usize,
    link: Scope<Self>,
    tags: Vec<String>,
    task: Timeout,
}

impl Slider {
    fn random() -> String {
        let random = Math::random();
        let multiplier = IMAGES.len() as f64;
        let number = (random * multiplier).floor() as usize;
        let link = IMAGES.get(number % IMAGES.len()).unwrap();

        format!("{}/{}", BASE, link)
    }

    fn class(&self, index: usize) -> &'static str {
        if self.current == index {
            "current"
        } else {
            "standby"
        }
    }

    fn schedule(&mut self, msg: Msg, seconds: u32) {
        let link = self.link.clone();
        let func = move || link.send_message(msg);
        let handle = Timeout::new(seconds * 1000, func);

        self.task = handle;
    }

    fn preload(&mut self) -> bool {
        let tags = self.tags.len();
        let prev = (self.current + tags - 1) % tags;

        self.tags[prev] = Self::random();
        self.schedule(Msg::Switch, 25);

        true
    }

    fn switch(&mut self) -> bool {
        let tags = self.tags.len();
        let next = (self.current + 1) % tags;

        self.current = next;
        self.schedule(Msg::Preload, 5);

        true
    }
}

impl Component for Slider {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let initial = (0..2).map(|_| Self::random()).collect();

        let link = ctx.link().clone();
        let func = move || link.send_message(Msg::Switch);
        let handle = Timeout::new(30 * 1000, func);

        Slider {
            current: 0,
            link: ctx.link().clone(),
            tags: initial,
            task: handle,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Preload => self.preload(),
            Msg::Switch => self.switch(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! { for self.tags.iter().enumerate().map(|(i, src)| {
            html! { <img class={{ self.class(i) }} src={{ src.clone() }} /> }
        })}
    }
}

struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Model {}
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <style>{STYLE}</style>
                <Slider />
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn initialize() {
    yew::start_app::<Model>();
}
