#!/usr/bin/env python3
"""Normalize connector data and expose an authenticated incremental change feed."""

from __future__ import annotations

import argparse
import hashlib
import hmac
import json
import os
import secrets
import sqlite3
import sys
import threading
import time
from dataclasses import asdict, dataclass
from datetime import datetime
from http import HTTPStatus
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import parse_qs, urlparse


SCHEMA = """
PRAGMA journal_mode=WAL;
PRAGMA synchronous=NORMAL;
CREATE TABLE IF NOT EXISTS documents (
  source TEXT NOT NULL,
  source_id TEXT NOT NULL,
  kind TEXT NOT NULL,
  title TEXT NOT NULL,
  body TEXT NOT NULL,
  people TEXT NOT NULL,
  occurred_at INTEGER NOT NULL,
  open_target TEXT,
  copy_text TEXT NOT NULL,
  metadata_json TEXT NOT NULL,
  fingerprint TEXT NOT NULL,
  updated_at INTEGER NOT NULL,
  deleted INTEGER NOT NULL DEFAULT 0,
  PRIMARY KEY (source, source_id)
);
CREATE TABLE IF NOT EXISTS changes (
  sequence INTEGER PRIMARY KEY AUTOINCREMENT,
  source TEXT NOT NULL,
  source_id TEXT NOT NULL,
  operation TEXT NOT NULL CHECK (operation IN ('upsert', 'delete')),
  created_at INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS changes_source_id ON changes(source, source_id);
"""


@dataclass(frozen=True)
class Document:
    source: str
    source_id: str
    kind: str
    title: str
    body: str
    people: str
    occurred_at: int
    open_target: str | None
    copy_text: str
    metadata_json: str

    def fingerprint(self) -> str:
        encoded = json.dumps(asdict(self), sort_keys=True, separators=(",", ":")).encode()
        return hashlib.sha256(encoded).hexdigest()


def open_store(path: Path) -> sqlite3.Connection:
    path.parent.mkdir(parents=True, exist_ok=True)
    connection = sqlite3.connect(path)
    connection.row_factory = sqlite3.Row
    connection.executescript(SCHEMA)
    return connection


def upsert_document(connection: sqlite3.Connection, document: Document) -> bool:
    fingerprint = document.fingerprint()
    existing = connection.execute(
        "SELECT fingerprint, deleted FROM documents WHERE source = ? AND source_id = ?",
        (document.source, document.source_id),
    ).fetchone()
    if existing and existing["fingerprint"] == fingerprint and not existing["deleted"]:
        return False
    now = int(time.time())
    values = asdict(document)
    connection.execute(
        """
        INSERT INTO documents(
          source, source_id, kind, title, body, people, occurred_at, open_target,
          copy_text, metadata_json, fingerprint, updated_at, deleted
        ) VALUES (
          :source, :source_id, :kind, :title, :body, :people, :occurred_at, :open_target,
          :copy_text, :metadata_json, :fingerprint, :updated_at, 0
        )
        ON CONFLICT(source, source_id) DO UPDATE SET
          kind=excluded.kind, title=excluded.title, body=excluded.body,
          people=excluded.people, occurred_at=excluded.occurred_at,
          open_target=excluded.open_target, copy_text=excluded.copy_text,
          metadata_json=excluded.metadata_json, fingerprint=excluded.fingerprint,
          updated_at=excluded.updated_at, deleted=0
        """,
        values | {"fingerprint": fingerprint, "updated_at": now},
    )
    connection.execute(
        "INSERT INTO changes(source, source_id, operation, created_at) VALUES (?, ?, 'upsert', ?)",
        (document.source, document.source_id, now),
    )
    return True


def timestamp_seconds(value: object) -> int:
    if isinstance(value, (int, float)):
        return int(value)
    if not value:
        return 0
    text = str(value)
    try:
        return int(float(text))
    except ValueError:
        try:
            return int(datetime.fromisoformat(text.replace("Z", "+00:00")).timestamp())
        except ValueError:
            return 0


def whatsapp_target(chat_jid: str) -> str | None:
    if chat_jid.endswith("@s.whatsapp.net"):
        phone = chat_jid.split("@", 1)[0].split(":", 1)[0]
        if phone.isdigit():
            return f"whatsapp://send?phone={phone}"
    return "whatsapp://"


def import_whatsapp(source_database: Path, collector_database: Path) -> int:
    if not source_database.exists():
        raise FileNotFoundError(f"WhatsApp message database not found: {source_database}")
    source = sqlite3.connect(f"file:{source_database}?mode=ro", uri=True)
    source.row_factory = sqlite3.Row
    target = open_store(collector_database)
    changed = 0
    try:
        rows = source.execute(
            """
            SELECT m.id, m.chat_jid, COALESCE(c.name, m.chat_jid) AS chat_name,
                   COALESCE(m.sender, '') AS sender, COALESCE(m.content, '') AS content,
                   m.timestamp, COALESCE(m.is_from_me, 0) AS is_from_me,
                   COALESCE(m.media_type, '') AS media_type,
                   COALESCE(m.filename, '') AS filename
            FROM messages m LEFT JOIN chats c ON c.jid = m.chat_jid
            ORDER BY m.timestamp, m.id
            """
        )
        with target:
            for row in rows:
                media = ""
                if row["media_type"]:
                    media = f"[{row['media_type']}{': ' + row['filename'] if row['filename'] else ''}]"
                body = row["content"] or media
                if not body:
                    continue
                sender = "Me" if row["is_from_me"] else row["sender"]
                document = Document(
                    source="whatsapp",
                    source_id=f"{row['chat_jid']}:{row['id']}",
                    kind="message",
                    title=row["chat_name"],
                    body=body,
                    people=sender,
                    occurred_at=timestamp_seconds(row["timestamp"]),
                    open_target=whatsapp_target(row["chat_jid"]),
                    copy_text=row["content"] or row["filename"] or body,
                    metadata_json=json.dumps(
                        {
                            "chat_jid": row["chat_jid"],
                            "message_id": row["id"],
                            "is_from_me": bool(row["is_from_me"]),
                            "media_type": row["media_type"] or None,
                            "filename": row["filename"] or None,
                        },
                        separators=(",", ":"),
                    ),
                )
                changed += int(upsert_document(target, document))
    finally:
        source.close()
        target.close()
    return changed


class CollectorServer(ThreadingHTTPServer):
    def __init__(self, address: tuple[str, int], database: Path, token: str):
        super().__init__(address, CollectorHandler)
        self.database = database
        self.token = token


class CollectorHandler(BaseHTTPRequestHandler):
    server: CollectorServer

    def log_message(self, format: str, *args: object) -> None:
        sys.stderr.write("collector: " + format % args + "\n")

    def reply(self, status: HTTPStatus, payload: object) -> None:
        body = json.dumps(payload, separators=(",", ":")).encode()
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(body)

    def authorized(self) -> bool:
        expected = f"Bearer {self.server.token}"
        actual = self.headers.get("Authorization", "")
        return hmac.compare_digest(actual, expected)

    def do_GET(self) -> None:  # noqa: N802
        if not self.authorized():
            self.reply(HTTPStatus.UNAUTHORIZED, {"error": "invalid bearer token"})
            return
        parsed = urlparse(self.path)
        if parsed.path == "/v1/health":
            connection = open_store(self.server.database)
            try:
                cursor = connection.execute("SELECT COALESCE(MAX(sequence), 0) FROM changes").fetchone()[0]
                counts = dict(connection.execute(
                    "SELECT source, COUNT(*) FROM documents WHERE deleted = 0 GROUP BY source"
                ).fetchall())
            finally:
                connection.close()
            self.reply(HTTPStatus.OK, {"ok": True, "cursor": cursor, "documents": counts})
            return
        if parsed.path == "/v1/changes":
            query = parse_qs(parsed.query)
            try:
                after = max(0, int(query.get("after", ["0"])[0]))
                limit = min(1000, max(1, int(query.get("limit", ["500"])[0])))
            except ValueError:
                self.reply(HTTPStatus.BAD_REQUEST, {"error": "after and limit must be integers"})
                return
            connection = open_store(self.server.database)
            try:
                rows = connection.execute(
                    """
                    SELECT c.sequence, c.operation, d.source, d.source_id, d.kind, d.title,
                           d.body, d.people, d.occurred_at, d.open_target, d.copy_text,
                           d.metadata_json, d.updated_at
                    FROM changes c
                    JOIN documents d ON d.source = c.source AND d.source_id = c.source_id
                    WHERE c.sequence > ? ORDER BY c.sequence LIMIT ?
                    """,
                    (after, limit),
                ).fetchall()
                changes = [dict(row) for row in rows]
                cursor = changes[-1]["sequence"] if changes else after
                more = connection.execute(
                    "SELECT EXISTS(SELECT 1 FROM changes WHERE sequence > ?)", (cursor,)
                ).fetchone()[0]
            finally:
                connection.close()
            self.reply(HTTPStatus.OK, {"cursor": cursor, "has_more": bool(more), "changes": changes})
            return
        self.reply(HTTPStatus.NOT_FOUND, {"error": "not found"})


def read_or_create_token(path: Path) -> str:
    if path.exists():
        token = path.read_text().strip()
        if token:
            return token
    path.parent.mkdir(parents=True, exist_ok=True)
    token = secrets.token_urlsafe(32)
    path.write_text(token + "\n")
    path.chmod(0o600)
    return token


def run_import_loop(source: Path, database: Path, interval: float, stop: threading.Event) -> None:
    while not stop.is_set():
        try:
            changed = import_whatsapp(source, database)
            if changed:
                print(f"Imported {changed} WhatsApp messages", file=sys.stderr)
        except (FileNotFoundError, sqlite3.Error) as error:
            print(str(error), file=sys.stderr)
        stop.wait(interval)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--database", type=Path, default=Path("data/collector.sqlite3"))
    subparsers = parser.add_subparsers(dest="command", required=True)

    import_parser = subparsers.add_parser("import-whatsapp")
    import_parser.add_argument("--source", type=Path, required=True)

    serve_parser = subparsers.add_parser("serve")
    serve_parser.add_argument("--host", default="127.0.0.1")
    serve_parser.add_argument("--port", type=int, default=8742)
    serve_parser.add_argument("--token-file", type=Path, default=Path("data/token"))
    serve_parser.add_argument("--whatsapp-database", type=Path)
    serve_parser.add_argument("--scan-interval", type=float, default=5.0)

    args = parser.parse_args()
    if args.command == "import-whatsapp":
        print(json.dumps({"changed": import_whatsapp(args.source, args.database)}))
        return

    token = read_or_create_token(args.token_file)
    stop = threading.Event()
    importer = None
    if args.whatsapp_database:
        importer = threading.Thread(
            target=run_import_loop,
            args=(args.whatsapp_database, args.database, args.scan_interval, stop),
            daemon=True,
        )
        importer.start()
    server = CollectorServer((args.host, args.port), args.database, token)
    print(f"Collector listening on http://{args.host}:{args.port}", file=sys.stderr)
    print(f"Token file: {args.token_file.resolve()}", file=sys.stderr)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        stop.set()
        server.server_close()
        if importer:
            importer.join(timeout=2)


if __name__ == "__main__":
    main()
