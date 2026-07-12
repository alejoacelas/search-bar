import { useCallback, useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import "./App.css";

type SearchResult = {
  path: string;
  name: string;
  parent: string;
  line: number | null;
  preview: string | null;
  kind: "name" | "content";
};

type IndexReport = { files: number; elapsedMs: number };

const HOME_LABEL = "Home";

function SearchIcon() {
  return <svg viewBox="0 0 24 24" aria-hidden="true"><circle cx="11" cy="11" r="6.5"/><path d="m16 16 4 4"/></svg>;
}

function FileIcon({ kind }: { kind: SearchResult["kind"] }) {
  return <span className={`file-icon ${kind}`}><svg viewBox="0 0 24 24" aria-hidden="true"><path d="M6 2.8h7l5 5V21H6z"/><path d="M13 2.8V8h5"/></svg></span>;
}

function App() {
  const [root, setRoot] = useState("");
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [selected, setSelected] = useState(0);
  const [searching, setSearching] = useState(false);
  const [indexing, setIndexing] = useState(false);
  const [indexedFiles, setIndexedFiles] = useState<number | null>(null);
  const [error, setError] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);
  const searchId = useRef(0);

  useEffect(() => {
    invoke<string>("home_directory").then(setRoot);
    inputRef.current?.focus();
  }, []);

  useEffect(() => {
    if (!root) return;
    let cancelled = false;
    setIndexing(true);
    setError("");
    const prepare = async () => {
      while (!cancelled) {
        try {
          const report = await invoke<IndexReport>("prepare_index", { root });
          if (!cancelled) setIndexedFiles(report.files);
          break;
        } catch (reason) {
          if (String(reason).includes("already running")) {
            await new Promise((resolve) => window.setTimeout(resolve, 250));
          } else {
            if (!cancelled) setError(String(reason));
            break;
          }
        }
      }
      if (!cancelled) setIndexing(false);
    };
    prepare();
    return () => { cancelled = true; };
  }, [root]);

  useEffect(() => {
    if (!query.trim() || !root) {
      searchId.current += 1;
      setResults([]);
      setSearching(false);
      return;
    }
    setSearching(true);
    const id = ++searchId.current;
    const request = window.setTimeout(() => {
      invoke<SearchResult[]>("search_files", { root, query, limit: 80 })
        .then((next) => { if (id === searchId.current) { setResults(next); setSelected(0); } })
        .catch((reason) => { if (id === searchId.current) setError(String(reason)); })
        .finally(() => { if (id === searchId.current) setSearching(false); });
    }, 120);
    return () => window.clearTimeout(request);
  }, [query, root]);

  const openSelected = useCallback(() => {
    const result = results[selected];
    if (result) invoke("open_file", { path: result.path });
  }, [results, selected]);

  const chooseFolder = async () => {
    const folder = await open({ directory: true, multiple: false, defaultPath: root });
    if (typeof folder === "string") setRoot(folder);
    inputRef.current?.focus();
  };

  const onKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === "ArrowDown") { event.preventDefault(); setSelected((i) => Math.min(i + 1, results.length - 1)); }
    if (event.key === "ArrowUp") { event.preventDefault(); setSelected((i) => Math.max(i - 1, 0)); }
    if (event.key === "Enter") { event.preventDefault(); openSelected(); }
    if (event.key === "Escape") { setQuery(""); }
  };

  const rootParts = root.split("/").filter(Boolean);
  const rootName = rootParts[rootParts.length - 1] || HOME_LABEL;

  return (
    <main className="shell">
      <section className="command-bar" aria-label="File search">
        <header className="search-row">
          <SearchIcon />
          <input ref={inputRef} value={query} onChange={(e) => setQuery(e.target.value)} onKeyDown={onKeyDown} placeholder="Search files…" aria-label="Search files" spellCheck={false} />
          {searching && <span className="spinner" aria-label="Searching" />}
        </header>

        <div className="scope-row">
          <button className="scope" onClick={chooseFolder} title={root}>
            <span className="folder">⌘</span><span>Search in {rootName}</span><span className="chevron">⌄</span>
          </button>
          <span className="count">
            {error ? "Search unavailable" : indexing ? "Indexing files…" : query ? `${results.length === 80 ? "80+" : results.length} results` : indexedFiles !== null ? `${indexedFiles.toLocaleString()} files` : ""}
          </span>
        </div>

        <div className="results" role="listbox">
          {!query && (
            <div className="empty">
              <span className="empty-icon"><SearchIcon /></span>
              <h1>Search your files</h1>
              <p>{indexing ? "Building a fast local index. You can start typing now." : "Find a file by its name or anything written inside it."}</p>
            </div>
          )}
          {query && !searching && results.length === 0 && (
            <div className="empty compact"><h1>{error ? "Search unavailable" : "No files found"}</h1><p>{error || "Try another phrase or choose a different folder."}</p></div>
          )}
          {results.map((result, index) => (
            <button key={`${result.path}:${result.line ?? 0}`} className={`result ${index === selected ? "selected" : ""}`} onMouseEnter={() => setSelected(index)} onDoubleClick={() => invoke("open_file", { path: result.path })} role="option" aria-selected={index === selected}>
              <FileIcon kind={result.kind} />
              <span className="result-copy">
                <span className="result-title">{result.name}{result.line ? <span className="line">:{result.line}</span> : null}</span>
                <span className="result-meta">{result.preview || result.parent}</span>
              </span>
              <span className="path">{result.parent}</span>
            </button>
          ))}
        </div>

        <footer>
          <span className="brand"><span className="brand-mark">S</span> Search Bar</span>
          <span className="shortcuts"><span>Choose folder</span><kbd>⌘</kbd><kbd>O</kbd><span>Open</span><kbd>↵</kbd></span>
        </footer>
      </section>
    </main>
  );
}

export default App;
