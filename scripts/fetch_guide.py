"""Fetch the OpenDART developer guide into local Markdown files.

Writes one file per endpoint plus an index to the output directory (default
`guide/`, which is gitignored: the guide text belongs to the FSS, so it is
kept out of the repository), and the intro, terms, notice, FAQ and API
change-notice pages under `site/`.

With `--snapshot`, also writes a JSON snapshot of the interface (endpoints,
request and response keys with their short labels, status codes, board
titles) with prose replaced by hashes. CI commits that file and diffs it to
detect guide changes.

Usage: python3 -I scripts/fetch_guide.py [out_dir] [--snapshot FILE] [--snapshot-only]
"""

import argparse
import hashlib
import html
import json
import re
import time
import urllib.parse
import urllib.request
from html.parser import HTMLParser
from pathlib import Path

BASE = "https://opendart.fss.or.kr"
GROUPS = ["DS001", "DS002", "DS003", "DS004", "DS005", "DS006"]
HEADERS = {"User-Agent": "Mozilla/5.0 (opendart-rs guide fetch)"}


def fetch(path, form=None):
    data = urllib.parse.urlencode(form).encode() if form else None
    req = urllib.request.Request(BASE + path, data=data, headers=HEADERS)
    with urllib.request.urlopen(req, timeout=30) as resp:
        body = resp.read().decode("utf-8")
    time.sleep(0.3)
    return body


# Site pages worth reading next to the guide.
PAGES = {
    "intro": "/intro/main.do",
    "service-list": "/intro/infoApiList.do",
    "terms": "/intro/terms.do",
}
# Bulletin boards: 공지사항, FAQ, 변동내역알림 (API change notices).
BOARDS = {
    "notice": "B0000000000000000001",
    "faq": "B0000000000000000002",
    "changes": "B0000000000000000004",
}
SKIP_IDS = ("header", "m_header", "lnb", "footer")


class PageText(HTMLParser):
    """Page text without the site header, menus, footer and popups."""

    def __init__(self):
        super().__init__()
        self.out = []
        self.stack = []  # (tag, skipping)

    def skipping(self):
        return any(skip for _, skip in self.stack)

    def handle_starttag(self, tag, attrs):
        if tag in ("br", "img", "input", "meta", "link", "hr", "col"):
            if tag == "br":
                self.out.append("\n")
            return
        ident = dict(attrs).get("id") or ""
        skip = tag in ("script", "style") or ident in SKIP_IDS or ident.startswith("layerPop")
        self.stack.append((tag, skip))
        if tag in ("p", "div", "li", "tr", "h1", "h2", "h3", "h4", "h5", "dt", "dd"):
            self.out.append("\n")

    def handle_endtag(self, tag):
        while self.stack:
            open_tag, _ = self.stack.pop()
            if open_tag == tag:
                break
        if tag in ("td", "th"):
            self.out.append(" | ")

    def handle_data(self, data):
        if not self.skipping():
            self.out.append(data)

    def text(self):
        body = clean("".join(self.out))
        body = body.split("서울특별시 영등포구 여의대로 38")[0]
        # The site menu is printed twice (desktop, mobile) and ends with
        # "탈퇴하기"; the page content follows its last occurrence.
        lines = body.split("\n")
        if "탈퇴하기" in lines:
            lines = lines[len(lines) - lines[::-1].index("탈퇴하기"):]
        return "\n".join(lines).strip()


def page_text(page):
    parser = PageText()
    parser.feed(page)
    parser.close()
    return drop_view_counts(parser.text())


def drop_view_counts(text):
    """Removes the 조회수 column of list pages (the number right after a
    YYYY-MM-DD line), which changes with every visit."""
    lines = text.split("\n")
    kept = [
        line for i, line in enumerate(lines)
        if not (i and re.fullmatch(r"\d{4}-\d{2}-\d{2}", lines[i - 1]) and re.fullmatch(r"[\d,]+", line))
    ]
    return "\n".join(kept)


def digest(text):
    """Short hash for prose, so the snapshot detects changes without
    copying the FSS's text."""
    return hashlib.sha256(text.encode("utf-8")).hexdigest()[:12]


def article_head(text):
    """(date, title) of an article: the title is the line before the first
    YYYY-MM-DD line."""
    lines = text.split("\n")
    for i, line in enumerate(lines):
        if re.fullmatch(r"\d{4}-\d{2}-\d{2}", line) and i:
            return line, lines[i - 1]
    return "", ""


def fetch_site(out):
    """Saves the site pages and boards under `out/site` (when `out` is set)
    and returns their snapshot entries."""
    site = out / "site" if out else None
    if site:
        site.mkdir(exist_ok=True)
    snapshot = {"pages": {}, "boards": {}}
    for name, path in PAGES.items():
        text = page_text(fetch(path))
        require(text.strip(), f"empty {name} page")
        snapshot["pages"][name] = digest(text)
        if site:
            (site / f"{name}.md").write_text(f"Source: {BASE}{path}\n\n{text}\n", encoding="utf-8")
        print("page", name)
    for name, bbs_id in BOARDS.items():
        board = site / name if site else None
        if board:
            board.mkdir(exist_ok=True)
        seen, page_index = [], 1
        while True:
            listing = fetch(f"/cop/bbs/selectArticleList.do?bbsId={bbs_id}&pageIndex={page_index}")
            ids = [i for i in re.findall(r'name="nttId" type="hidden" value="(\d+)"', listing) if i not in seen]
            if not ids:
                break
            seen += ids
            page_index += 1
        require(seen, f"no articles on the {name} board")
        entries = []
        for ntt_id in seen:
            text = page_text(fetch("/cop/bbs/selectArticleDetail.do", {"nttId": ntt_id, "bbsId": bbs_id}))
            date, title = article_head(text)
            # Maintenance notices come every month or two and say nothing
            # about the API itself.
            if name == "notice" and "시스템 점검" in title:
                continue
            entries.append({"ntt_id": ntt_id, "date": date, "title": title, "body": digest(text)})
            if board:
                (board / f"{ntt_id}.md").write_text(f"Source: {BASE}/cop/bbs/selectArticleDetail.do (bbsId={bbs_id}, nttId={ntt_id})\n\n{text}\n", encoding="utf-8")
        snapshot["boards"][name] = sorted(entries, key=lambda e: int(e["ntt_id"]))
        print("board", name, len(seen), "articles")
    return snapshot


def clean(text):
    text = html.unescape(text).replace("\xa0", " ")
    lines = [re.sub(r"[ \t\r]+", " ", line).strip() for line in text.split("\n")]
    return "\n".join(line for line in lines if line)


class Sections(HTMLParser):
    """Collects `<h5>` sections; each holds tables (rows of cell texts) and
    loose text outside tables."""

    def __init__(self):
        super().__init__(convert_charrefs=False)
        self.sections = []  # [{"title", "tables": [[[cell]]], "text": [str]}]
        self.in_h5 = False
        self.h5 = ""
        self.table_depth = 0
        self.row = None
        self.cell = None
        self.skip = 0  # inside <script>/<style>
        self.loose = ""

    def current(self):
        return self.sections[-1] if self.sections else None

    def handle_starttag(self, tag, attrs):
        if tag in ("script", "style"):
            self.skip += 1
        elif tag == "h5":
            self.flush_loose()
            self.in_h5, self.h5 = True, ""
        elif tag == "table":
            self.flush_loose()
            self.table_depth += 1
            if self.current() is not None and self.table_depth == 1:
                self.current()["tables"].append([])
        elif tag == "tr" and self.table_depth:
            self.row = []
        elif tag in ("td", "th") and self.row is not None:
            self.cell = ""
        elif tag == "br":
            self.feed_text("\n")
        elif tag in ("p", "li", "div") and not self.table_depth:
            self.loose += "\n"

    def handle_endtag(self, tag):
        if tag in ("script", "style"):
            self.skip = max(0, self.skip - 1)
        elif tag == "h5" and self.in_h5:
            self.in_h5 = False
            self.sections.append({"title": clean(self.h5), "tables": [], "text": []})
        elif tag == "table" and self.table_depth:
            self.table_depth -= 1
        elif tag in ("td", "th") and self.cell is not None:
            self.row.append(clean(self.cell))
            self.cell = None
        elif tag == "tr" and self.row is not None:
            if self.current() is not None and self.current()["tables"] and any(self.row):
                self.current()["tables"][-1].append(self.row)
            self.row = None

    def feed_text(self, data):
        if self.skip:
            return
        if self.in_h5:
            self.h5 += data
        elif self.cell is not None:
            self.cell += data
        elif not self.table_depth:
            self.loose += data

    def handle_data(self, data):
        self.feed_text(data)

    def handle_entityref(self, name):
        self.feed_text(f"&{name};")

    def handle_charref(self, name):
        self.feed_text(f"&#{name};")

    def flush_loose(self):
        text = clean(self.loose)
        self.loose = ""
        if text and self.current() is not None:
            self.current()["text"].append(text)

    def close(self):
        super().close()
        self.flush_loose()


def md_cell(text):
    return text.replace("|", "\\|").replace("\n", "<br>")


def md_table(rows):
    width = max(len(r) for r in rows)
    rows = [r + [""] * (width - len(r)) for r in rows]
    out = ["| " + " | ".join(md_cell(c) for c in rows[0]) + " |"]
    out.append("|" + "---|" * width)
    out += ["| " + " | ".join(md_cell(c) for c in r) + " |" for r in rows[1:]]
    return "\n".join(out)


def parse_index(group_html, group):
    """Rows of the group's API table: (apiId, name, description)."""
    rows = []
    for tr in re.findall(r"<tr>(.*?)</tr>", group_html, flags=re.S):
        m = re.search(rf"apiGrpCd={group}&(?:amp;)?apiId=(\d+)", tr)
        if not m:
            continue
        cells = [clean(re.sub(r"<[^>]+>", "", c)) for c in re.findall(r"<td[^>]*>(.*?)</td>", tr, flags=re.S)]
        rows.append({"api_id": m.group(1), "name": cells[1], "summary": cells[2]})
    titles = [clean(t) for t in re.findall(rf'apiGrpCd={group}"[^>]*>([^<]+)</a>', group_html)]
    titles = [t for t in titles if t != "개발가이드"]
    return (titles[0] if titles else group), rows


def api_snapshot(sections):
    """The interface facts of one guide page: endpoints, request and
    response keys with their short labels, and hashes of the prose."""
    entry = {"endpoints": [], "request": [], "response": [], "messages": [], "other": {}}
    for section in sections:
        rows = [r for table in section["tables"] for r in table[1:]]  # skip header rows
        title = section["title"]
        if title == "기본 정보":
            entry["endpoints"] = [{"method": r[0], "url": r[1], "format": r[3]} for r in rows if len(r) >= 4]
        elif title == "요청 인자":
            entry["request"] = [
                {"key": r[0], "label": r[1], "type": r[2], "required": r[3], "desc": digest(r[4])}
                for r in rows if len(r) >= 5
            ]
        elif title == "응답 결과":
            entry["response"] = [
                {"key": r[0], "label": r[1] if len(r) > 1 else "", "desc": digest(r[2] if len(r) > 2 else "")}
                for r in rows
            ]
        elif title == "메시지 설명":
            # This table has no header row.
            all_rows = [r for table in section["tables"] for r in table]
            entry["messages"] = sorted({m for r in all_rows for m in re.findall(r"- (\d{3}) :", " ".join(r))})
        elif title != "OpenAPI 테스트":
            entry["other"][title] = digest(json.dumps(section["tables"], ensure_ascii=False))
    return entry


def slug(text):
    return re.sub(r"[^0-9A-Za-z가-힣]+", "-", text).strip("-")


def require(ok, what):
    """Stops before anything is written when a page does not look like the
    guide, e.g. a maintenance page or changed markup. A partial snapshot
    would otherwise report every missing API as a guide change."""
    if not ok:
        raise SystemExit(f"unexpected page content: {what}")


def main():
    args = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    args.add_argument("out", nargs="?", default="guide", help="directory for the Markdown copy")
    args.add_argument("--snapshot", type=Path, help="also write the interface snapshot JSON here")
    args.add_argument("--snapshot-only", action="store_true", help="write only the snapshot")
    opts = args.parse_args()
    out = None if opts.snapshot_only else Path(opts.out)
    if out:
        out.mkdir(parents=True, exist_ok=True)
    index, apis_snapshot = [], []
    for group in GROUPS:
        group_html = fetch(f"/guide/main.do?apiGrpCd={group}")
        group_name, apis = parse_index(group_html, group)
        require(apis and group_name != group, f"no APIs or group name in the {group} index")
        group_dir = out / f"{group}-{slug(group_name)}" if out else None
        if group_dir:
            group_dir.mkdir(exist_ok=True)
        for api in apis:
            page = fetch(f"/guide/detail.do?apiGrpCd={group}&apiId={api['api_id']}")
            parser = Sections()
            parser.feed(page)
            parser.close()
            sections = [s for s in parser.sections if s["tables"] or s["text"]]
            urls = sorted(set(re.findall(r"https://opendart\.fss\.or\.kr/api/[A-Za-z0-9]+\.(?:json|xml)", page)))
            entry = api_snapshot(sections)
            for part in ("endpoints", "request", "response", "messages"):
                require(entry[part], f"no {part} on {group} {api['api_id']}")
            require(urls, f"no endpoint URL on {group} {api['api_id']}")
            apis_snapshot.append({
                "group": group, "group_name": group_name, "api_id": api["api_id"], "name": api["name"],
                "summary": digest(api["summary"]), **entry,
            })
            print(group, api["api_id"], api["name"], " ".join(u.rsplit("/", 1)[1] for u in urls))
            if not out:
                continue
            doc = [
                f"# {api['name']} ({group} {api['api_id']})",
                "",
                f"Source: {BASE}/guide/detail.do?apiGrpCd={group}&apiId={api['api_id']}",
                "",
                api["summary"],
            ]
            for section in sections:
                doc += ["", f"## {section['title']}"]
                for table in section["tables"]:
                    if table:
                        doc += ["", md_table(table)]
                for text in section["text"]:
                    doc += ["", text]
            path = group_dir / f"{api['api_id']}-{slug(api['name'])}.md"
            path.write_text("\n".join(doc) + "\n", encoding="utf-8")
            index.append({"group": group, "group_name": group_name, **api, "urls": urls,
                          "file": str(path.relative_to(out))})
    if out:
        (out / "index.json").write_text(json.dumps(index, ensure_ascii=False, indent=1), encoding="utf-8")
        lines = ["# OpenDART 개발가이드 index", "", "| Group | apiId | API | Endpoints | File |", "|---|---|---|---|---|"]
        for e in index:
            eps = ", ".join(u.rsplit("/", 1)[1] for u in e["urls"])
            lines.append(f"| {e['group']} {e['group_name']} | {e['api_id']} | {e['name']} | {eps} | [{e['file']}]({e['file']}) |")
        (out / "README.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(len(apis_snapshot), "APIs")
    site = fetch_site(out)
    if opts.snapshot:
        snapshot = {"apis": apis_snapshot, **site}
        opts.snapshot.parent.mkdir(parents=True, exist_ok=True)
        opts.snapshot.write_text(json.dumps(snapshot, ensure_ascii=False, indent=1, sort_keys=True) + "\n", encoding="utf-8")
        print("snapshot", opts.snapshot)


if __name__ == "__main__":
    main()
