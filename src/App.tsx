import { useRef } from "react";
import { useFocusHandoff } from "./hooks/useFocusHandoff";

function App() {
  const field = useRef<HTMLInputElement>(null);
  useFocusHandoff(field);

  return (
    <main className="flex h-full items-center justify-center p-2 select-none">
      <div className="flex h-10 w-full items-center gap-2 rounded-full bg-neutral-900/90 px-4 text-sm font-medium text-neutral-100 ring-1 ring-white/10">
        <span className="size-2 shrink-0 rounded-full bg-sky-400" />
        {/* TODO(REK-260): replace the focus test field with the summoned panel. */}
        <input
          ref={field}
          aria-label="Focus test"
          placeholder="⌥Space, type, Esc"
          autoComplete="off"
          autoCorrect="off"
          autoCapitalize="off"
          spellCheck={false}
          className="min-w-0 flex-1 bg-transparent outline-none placeholder:text-neutral-500"
        />
      </div>
    </main>
  );
}

export default App;
