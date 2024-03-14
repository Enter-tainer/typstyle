import './style.css'
import { pretty_print_wasm } from "typst_geshihua"
import van from "vanjs-core"
const { div, textarea, input, label } = van.tags;

const App = () => {
  const input_val = van.state("")
  const output = van.state("")
  const columns = van.state(80)
  van.derive(
    () => {
      try {
        output.val = pretty_print_wasm(input_val.val, columns.val)
      } catch (e) {
        output.val = e as string
      }
    }
  )
  return div(
    textarea({
      class: "mitex-input",
      placeholder: "Put typst code here",
      value: input_val,
      autofocus: true,
      rows: 30,
      oninput(event: Event) {
        input_val.val = (event.target! as HTMLInputElement).value;
      },
    }),
    textarea({
      class: "mitex-output",
      value: output,
      readOnly: true,
      placeholder: "Output",
      rows: 30,
      onfocus: (event: Event) =>
        (event.target! as HTMLTextAreaElement).select(),
    })
    , div(label("Columns: "),
      input({
        type: "number",
        value: columns,
        oninput(event: Event) {
          columns.val = parseInt((event.target! as HTMLInputElement).value)
        }
      })
    )
  )
}

const appElement = document.querySelector<Element>('#app');
if (appElement) {
  van.add(appElement, App());
}
