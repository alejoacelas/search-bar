import json
import sqlite3
import tempfile
import threading
import unittest
from pathlib import Path
from urllib.error import HTTPError
from urllib.request import Request, urlopen

from collector import CollectorServer, import_whatsapp, open_store


class WhatsAppImportTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        root = Path(self.temp.name)
        self.source = root / "messages.db"
        self.target = root / "collector.db"
        connection = sqlite3.connect(self.source)
        connection.executescript(
            """
            CREATE TABLE chats(jid TEXT PRIMARY KEY, name TEXT, last_message_time TIMESTAMP);
            CREATE TABLE messages(
              id TEXT, chat_jid TEXT, sender TEXT, content TEXT, timestamp TIMESTAMP,
              is_from_me BOOLEAN, media_type TEXT, filename TEXT,
              PRIMARY KEY(id, chat_jid)
            );
            INSERT INTO chats VALUES ('491234@s.whatsapp.net', 'Ada', '2026-07-12 10:00:00');
            INSERT INTO messages VALUES (
              'message-1', '491234@s.whatsapp.net', '491234', 'meeting at five',
              '2026-07-12 10:00:00', 0, '', ''
            );
            """
        )
        connection.close()

    def tearDown(self):
        self.temp.cleanup()

    def test_import_is_incremental_and_emits_normalized_change(self):
        self.assertEqual(import_whatsapp(self.source, self.target), 1)
        self.assertEqual(import_whatsapp(self.source, self.target), 0)
        connection = open_store(self.target)
        row = connection.execute("SELECT * FROM documents").fetchone()
        change_count = connection.execute("SELECT COUNT(*) FROM changes").fetchone()[0]
        connection.close()
        self.assertEqual(row["title"], "Ada")
        self.assertEqual(row["body"], "meeting at five")
        self.assertEqual(row["open_target"], "whatsapp://send?phone=491234")
        self.assertEqual(json.loads(row["metadata_json"])["message_id"], "message-1")
        self.assertEqual(change_count, 1)

    def test_changed_message_emits_second_change(self):
        import_whatsapp(self.source, self.target)
        connection = sqlite3.connect(self.source)
        connection.execute("UPDATE messages SET content = 'meeting at six'")
        connection.commit()
        connection.close()
        self.assertEqual(import_whatsapp(self.source, self.target), 1)
        connection = open_store(self.target)
        self.assertEqual(connection.execute("SELECT COUNT(*) FROM changes").fetchone()[0], 2)
        connection.close()


class ApiTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.database = Path(self.temp.name) / "collector.db"
        open_store(self.database).close()
        self.server = CollectorServer(("127.0.0.1", 0), self.database, "secret-token")
        self.thread = threading.Thread(target=self.server.serve_forever, daemon=True)
        self.thread.start()

    def tearDown(self):
        self.server.shutdown()
        self.server.server_close()
        self.thread.join()
        self.temp.cleanup()

    def test_health_requires_token(self):
        url = f"http://127.0.0.1:{self.server.server_port}/v1/health"
        with self.assertRaises(HTTPError) as raised:
            urlopen(url)
        self.assertEqual(raised.exception.code, 401)
        request = Request(url, headers={"Authorization": "Bearer secret-token"})
        with urlopen(request) as response:
            payload = json.load(response)
        self.assertTrue(payload["ok"])
        self.assertEqual(payload["cursor"], 0)


if __name__ == "__main__":
    unittest.main()
