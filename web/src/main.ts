import './style.css'
import { pretty_print_wasm } from "typst_geshihua"
import van from "vanjs-core"
const { div, textarea, input, label, p, a } = van.tags;

const App = () => {
  const input_val = van.state("")
  const output = van.state("")
  const error_message = van.state("")
  const columns = van.state(80)
  van.derive(
    () => {
      try {
        output.val = pretty_print_wasm(input_val.val, columns.val)
        error_message.val = ""
        if (pretty_print_wasm(output.val, columns.val) !== output.val) {
          throw new Error("Format doesn't converge! This means formatting the output again will result in a different output. This is a bug in the formatter. Please report it to https://github.com/Enter-tainer/typst-geshihua with the input code.")
        }
      } catch (e: any) {
        error_message.val = e.message
      }
    }
  )
  return div(
    p("Powered by ", a({
      href: "https://github.com/Enter-tainer/typst-geshihua"
    }, "Typst Geshihua")),
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
    ), div(p({ class: "error-message" }, error_message.val))
  )
}

const appElement = document.querySelector<Element>('#app');
if (appElement) {
  van.add(appElement, App());
}
