use leptos::{*, html::Input};

const TYPE_AREA_CLASS: &str = "h-24 w-full fixed bottom-0 flex justify-center items-center p-5 border-t z-10 shadow-lg";
const TYPE_AREA_CLASS_LIGHT: &str = "bg-gray-50 border-gray-200";
const TYPE_AREA_CLASS_DARK: &str = "bg-gray-800 border-gray-600";

const TEXT_AREA_CLASS: &str = "w-2/3 p-4 border rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500";
const TEXT_AREA_CLASS_LIGHT: &str = "border-gray-300 text-gray-800 placeholder-gray-500";
const TEXT_AREA_CLASS_DARK: &str = "bg-gray-700 border-gray-600 text-gray-200 placeholder-gray-400";

const BUTTON_CLASS: &str = "h-full p-4 rounded-lg cursor-pointer hover:opacity-90 transition-opacity";
const BUTTON_CLASS_LIGHT: &str = "bg-blue-600 text-white hover:bg-blue-700";
const BUTTON_CLASS_DARK: &str = "bg-green-600 text-white hover:bg-green-700";

#[component]
pub fn TypeArea(send: Action<String, Result<(), ServerFnError>>) -> impl IntoView {
    let dark_mode = use_context::<ReadSignal<bool>>().expect("should be able to get dark mode state");

    let type_area_class = Signal::derive(move || {
      if dark_mode.get() {
        format!("{TYPE_AREA_CLASS} {TYPE_AREA_CLASS_DARK}")
      } else {
        format!("{TYPE_AREA_CLASS} {TYPE_AREA_CLASS_LIGHT}")
      }
    });

    let text_area_class = Signal::derive(move || {
      if dark_mode.get() {
        format!("{TEXT_AREA_CLASS} {TEXT_AREA_CLASS_DARK}")
      } else {
        format!("{TEXT_AREA_CLASS} {TEXT_AREA_CLASS_LIGHT}")
      }
    });

    let button_class = Signal::derive(move || {
      if dark_mode.get() {
        format!("{BUTTON_CLASS} {BUTTON_CLASS_DARK}")
      } else {
        format!("{BUTTON_CLASS} {BUTTON_CLASS_LIGHT}")
      }
    });

    let input_ref = create_node_ref::<Input>();
    view!{
        <div class={type_area_class.get()}>
           <form class="w-full flex justify-center items-center gap-4" on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("input to exist");
                send.dispatch(input.value());
                input.set_value("");
           }
           >
                <input class={text_area_class.get()} type="text" placeholder="Enter your prompt" node_ref=input_ref/>
                <button class={button_class.get()} type="submit">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12h15m0 0l-6.75-6.75M19.5 12l-6.75 6.75" />
                    </svg>
                </button>
           </form>
        </div>
    }
}
