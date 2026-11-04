import { type RefObject, useEffect } from "react";
import { commands } from "../bindings";

export function useFocusHandoff(field: RefObject<HTMLInputElement | null>) {
  useEffect(() => {
    const input = field.current;
    if (!input) return;
    let holdsKeys = document.hasFocus();

    const moveCaretToEnd = () =>
      input.setSelectionRange(input.value.length, input.value.length);
    const take = () => {
      holdsKeys = true;
      input.focus();
      moveCaretToEnd();
    };
    const drop = () => {
      holdsKeys = false;
      moveCaretToEnd();
      input.blur();
    };
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key !== "Escape") return;
      event.preventDefault();
      drop();
      void commands.releaseFocus();
    };
    const onFieldFocus = () => {
      if (!holdsKeys) drop();
    };

    window.addEventListener("focus", take);
    window.addEventListener("blur", drop);
    window.addEventListener("keydown", onKeyDown);
    input.addEventListener("focus", onFieldFocus);
    return () => {
      window.removeEventListener("focus", take);
      window.removeEventListener("blur", drop);
      window.removeEventListener("keydown", onKeyDown);
      input.removeEventListener("focus", onFieldFocus);
    };
  }, [field]);
}
