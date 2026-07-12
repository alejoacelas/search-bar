import { useCallback, useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import "./App.css";

type SearchResult = {
  id: string;
  source: string;
  kind: string;
  title: string;
  subtitle: string;
  preview: string | null;
  openTarget: string | null;
  copyText: string;
  occurredAt: number | null;
};

type IndexReport = { files: number; elapsedMs: number };
type SyncReport = { configured: boolean; imported: number; cursor: number; documents: number; error: string | null };

const HOME_LABEL = "Home";

function SearchIcon() {
  return <svg viewBox="0 0 24 24" aria-hidden="true"><circle cx="11" cy="11" r="6.5"/><path d="m16 16 4 4"/></svg>;
}

function ResultIcon({ source }: { source: string }) {
  if (source === "whatsapp") return <span className="file-icon whatsapp">W</span>;
  return <span className="file-icon file"><svg viewBox="0 0 24 24" aria-hidden="true"><path d="M6 2.8h7l5 5V21H6z"/><path d="M13 2.8V8h5"/></svg></span>;
}

function resultContext(result: SearchResult) {
  const date = result.occurredAt ? new Date(result.occurredAt * 1000).toLocaleDateString(undefined, { month: "short", day: "numeric", year: "numeric" }) : "";
  return [result.subtitle, date].filter(Boolean).join(" · ");
}

function App() {
  const [root, setRoot] = useState("");
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [selected, setSelected] = useState(0);
  const [searching, setSearching] = useState(false);
  const [indexing, setIndexing] = useState(false);
  const [indexedFiles, setIndexedFiles] = useState<number | null>(null);
  const [indexRevision, setIndexRevision] = useState(0);
  const [collector, setCollector] = useState<SyncReport | null>(null);
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
          if (!cancelled) {
            setIndexedFiles(report.files);
            setIndexRevision((revision) => revision + 1);
          }
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
    let cancelled = false;
    const sync = () => invoke<SyncReport>("sync_collector")
      .then((report) => {
        if (!cancelled) {
          setCollector(report);
          if (report.imported) setIndexRevision((revision) => revision + 1);
        }
      })
      .catch((reason) => {
        if (!cancelled && !String(reason).includes("already running")) {
          setCollector((current) => current ? { ...current, error: String(reason) } : null);
        }
      });
    sync();
    const timer = window.setInterval(sync, 10_000);
    return () => { cancelled = true; window.clearInterval(timer); };
  }, []);

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
  }, [query, root, indexRevision]);

  const openSelected = useCallback(() => {
    const result = results[selected];
    if (result?.openTarget) invoke("open_result", { target: result.openTarget });
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
          <input ref={inputRef} value={query} onChange={(e) => setQuery(e.target.value)} onKeyDown={onKeyDown} placeholder="Search everything…" aria-label="Search" spellCheck={false} />
          {searching && <span className="spinner" aria-label="Searching" />}
        </header>

        <div className="scope-row">
          <button className="scope" onClick={chooseFolder} title={root}>
            <span className="folder">⌘</span><span>Search in {rootName}</span><span className="chevron">⌄</span>
          </button>
          <span className="count">
            {error ? "Search unavailable" : collector?.error ? "Collector offline · local results available" : indexing ? "Indexing files…" : query ? `${results.length === 80 ? "80+" : results.length} results` : indexedFiles !== null ? `${indexedFiles.toLocaleString()} files${collector?.configured ? ` · ${collector.documents.toLocaleString()} connected items` : ""}` : ""}
          </span>
        </div>

        <div className="results" role="listbox">
          {!query && (
            <div className="empty">
              <span className="empty-icon"><SearchIcon /></span>
              <h1>Search everything</h1>
              <p>{indexing ? "Building a fast local index. You can start typing now." : collector?.configured ? "Find files and connected messages without waiting for the network." : "Find a file by its name or anything written inside it."}</p>
            </div>
          )}
          {query && !searching && results.length === 0 && (
            <div className="empty compact"><h1>{error ? "Search unavailable" : indexing ? "Indexing files…" : "No files found"}</h1><p>{error || (indexing ? "Results will appear as soon as the local index is ready." : "Try another phrase or choose a different folder.")}</p></div>
          )}
          {results.map((result, index) => (
            <button key={result.id} className={`result ${index === selected ? "selected" : ""}`} onMouseEnter={() => setSelected(index)} onDoubleClick={() => result.openTarget && invoke("open_result", { target: result.openTarget })} role="option" aria-selected={index === selected}>
              <ResultIcon source={result.source} />
              <span className="result-copy">
                <span className="result-title">{result.title}</span>
                <span className="result-meta">{result.preview || resultContext(result)}</span>
              </span>
              <span className="path">{result.source === "file" ? result.subtitle : resultContext(result)}</span>
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
