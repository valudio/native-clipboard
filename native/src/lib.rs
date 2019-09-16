#[macro_use]
extern crate neon;
extern crate clipboard;
extern crate enigo;

use neon::prelude::*;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use enigo::*;

fn get_from_clipboard(mut cx: FunctionContext) -> JsResult<JsString> {
  let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
  Ok(cx.string(ctx.get_contents().unwrap()))
}

fn set_to_clipboard(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
  let x = cx.argument::<JsString>(0)?.value();
  ctx.set_contents(x).unwrap();
  Ok(cx.undefined())
}

fn set_selection_to_clipboard(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let mut enigo = Enigo::new();
  let c_key = if cfg!(windows) { 43 } else { 8 };
  enigo.key_down(Key::Meta);
  enigo.key_click(Key::Raw(c_key));
  enigo.key_up(Key::Meta);
  Ok(cx.undefined())
}

register_module!(mut cx, {
  cx.export_function("getFromClipboard", get_from_clipboard)?;
  cx.export_function("setToClipboard", set_to_clipboard)?;
  cx.export_function("setSelectionToClipboard", set_selection_to_clipboard)?;
  Ok(())
});
