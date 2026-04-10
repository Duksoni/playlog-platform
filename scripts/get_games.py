import os
import re
import time
import requests
from bs4 import BeautifulSoup
from dotenv import load_dotenv
from pathlib import Path


load_dotenv()
API_KEY = os.getenv("RAWG_API_KEY")
BASE_URL = "https://api.rawg.io/api"
HEADERS = {"User-Agent": "Playlog-Academic-Project"}

if not API_KEY:
    raise ValueError("RAWG_API_KEY not found in environment variables.")

TOP_GAMES = 200
PAGE_SIZE = 40
TRAILER_LIMIT = 10
MAX_SCREENSHOTS = 3
REQUEST_DELAY = 0.25

MEDIA_ROOT = Path("media")
SQL_ROOT = Path("sql_seed")

MEDIA_ROOT.mkdir(exist_ok=True)
SQL_ROOT.mkdir(exist_ok=True)


def api_get(endpoint, params=None):
    if params is None:
        params = {}
    params["key"] = API_KEY
    response = requests.get(f"{BASE_URL}{endpoint}", params=params, headers=HEADERS)
    response.raise_for_status()
    time.sleep(REQUEST_DELAY)
    return response.json()


def fetch_all(endpoint):
    """Paginate through an entire RAWG endpoint and return all results."""
    print(f"Fetching all {endpoint}...")
    results = []
    page = 1

    while True:
        data = api_get(f"/{endpoint}", {"page": page, "page_size": 40})
        results.extend(data["results"])
        if not data["next"]:
            break
        page += 1

    print(f"  -> {len(results)} {endpoint} fetched")
    return results


def download_file(url, path):
    if not url:
        return False
    try:
        r = requests.get(url, timeout=10, headers=HEADERS)
        if r.status_code == 200:
            with open(path, "wb") as f:
                f.write(r.content)
            return True
    except Exception:
        pass
    return False


def sanitize(text):
    if not text:
        return ""
    return text.replace("'", "''")


def clean_description(raw_html):
    """Strip HTML tags and clean up whitespace from game descriptions."""
    if not raw_html:
        return ""
    soup = BeautifulSoup(raw_html, "html.parser")
    text = soup.get_text(separator=" ", strip=True)
    text = re.sub(r"\s+", " ", text).strip()
    return text


def is_ascii(text):
    """Return True if the text contains only ASCII characters (filters Cyrillic)."""
    try:
        text.encode("ascii")
        return True
    except UnicodeEncodeError:
        return False


def to_title_case(text):
    return text.strip().title()


def write_sql(filename, lines):
    path = SQL_ROOT / filename
    with open(path, "w", encoding="utf-8") as f:
        f.write("\n".join(lines))
    print(f"  -> Written {path} ({len(lines)} statements)")


def fetch_top_games():
    games = []

    pages = TOP_GAMES // PAGE_SIZE
    for page in range(1, pages + 1):
        print(f"Fetching page {page}")
        data = api_get(
            "/games",
            {
                "ordering": "-added",
                "page_size": PAGE_SIZE,
                "page": page,
            },
        )
        games.extend(data["results"])

    return games


def main():
    # ----------------------
    # 1. Platforms and genres — not a lot of them, they can be fetched in full
    # ----------------------
    print("\n[1/4] Fetching platforms and genres...")

    platforms = fetch_all("platforms")
    genres = fetch_all("genres")

    write_sql(
        "01_seed_platforms.sql",
        [
            f"INSERT INTO platforms (rawg_id, name) VALUES ({p['id']}, '{sanitize(p['name'])}') ON CONFLICT (rawg_id) DO NOTHING;"
            for p in platforms
        ],
    )

    write_sql(
        "02_seed_genres.sql",
        [
            f"INSERT INTO genres (rawg_id, name) VALUES ({g['id']}, '{sanitize(g['name'])}') ON CONFLICT (rawg_id) DO NOTHING;"
            for g in genres
        ],
    )

    # ----------------------
    # 2. Fetch game list
    # ----------------------
    print("\n[2/4] Fetching top games list...")
    games_list = fetch_top_games()

    all_developers = {}
    all_publishers = {}
    all_tags = {}

    games_sql = []
    game_dev_rel = []
    game_pub_rel = []
    game_tag_rel = []
    game_genre_rel = []
    game_platform_rel = []

    trailers_found = 0

    # ----------------------
    # 3. Process each game — fetch details, collect entities, download media
    # ----------------------
    print(f"\n[3/4] Processing {len(games_list)} games...")

    for index, game in enumerate(games_list):
        rawg_id = game["id"]
        internal_id = index + 1

        print(f"  [{internal_id}/{TOP_GAMES}] {game['name']}")

        details = api_get(f"/games/{rawg_id}")

        # Developers
        for dev in details.get("developers", []):
            all_developers[dev["id"]] = dev["name"]
            game_dev_rel.append((rawg_id, dev["id"]))

        # Publishers
        for pub in details.get("publishers", []):
            all_publishers[pub["id"]] = pub["name"]
            game_pub_rel.append((rawg_id, pub["id"]))

        # Tags
        for tag in details.get("tags", []):
            tag_name = tag["name"]
            if not is_ascii(tag_name):
                continue
            all_tags[tag["id"]] = to_title_case(tag_name)
            game_tag_rel.append((rawg_id, tag["id"]))

        # Genres
        for genre in details.get("genres", []):
            game_genre_rel.append((rawg_id, genre["id"]))

        # Platforms — detail response nests platform object under a 'platform' key
        for platform_entry in details.get("platforms", []):
            platform = platform_entry.get("platform", {})
            if platform:
                game_platform_rel.append((rawg_id, platform["id"]))

        # Media — stored under internal id so folder name matches games.id
        game_dir = MEDIA_ROOT / str(internal_id)
        game_dir.mkdir(exist_ok=True)

        download_file(details.get("background_image"), game_dir / "cover.jpg")

        screenshots = api_get(f"/games/{rawg_id}/screenshots")
        for i, shot in enumerate(screenshots.get("results", [])[:MAX_SCREENSHOTS]):
            download_file(shot["image"], game_dir / f"screenshot_{i + 1}.jpg")

        if trailers_found < TRAILER_LIMIT:
            movies = api_get(f"/games/{rawg_id}/movies")
            if movies.get("results"):
                video_url = movies["results"][0].get("data", {}).get("max")
                if download_file(video_url, game_dir / "trailer.mp4"):
                    trailers_found += 1
                    print(f"    -> Trailer saved ({trailers_found}/{TRAILER_LIMIT})")

        # Game SQL row with explicit id
        released = details.get("released")
        released_sql = f"'{released}'" if released else "NULL"
        description = clean_description(
            details.get("description") or details.get("description_raw")
        )

        games_sql.append(
            f"INSERT INTO games (id, rawg_id, name, description, released, website, draft)\n"
            f"OVERRIDING SYSTEM VALUE\n"
            f"VALUES (\n"
            f"    {internal_id},\n"
            f"    {rawg_id},\n"
            f"    '{sanitize(details.get('name'))}',\n"
            f"    '{sanitize(description)}',\n"
            f"    {released_sql},\n"
            f"    '{sanitize(details.get('website') or '')}',\n"
            f"    FALSE\n"
            f");"
        )

    # Reset sequence after explicit id inserts so future auto-inserts don't collide
    games_sql.append(f"\nSELECT setval('games_id_seq', {TOP_GAMES}, true);")

    # ----------------------
    # 4. Write all SQL files
    # ----------------------
    print("\n[4/4] Writing SQL seed files...")

    write_sql(
        "03_seed_developers.sql",
        [
            f"INSERT INTO developers (rawg_id, name) VALUES ({did}, '{sanitize(name)}') ON CONFLICT (rawg_id) DO NOTHING;"
            for did, name in all_developers.items()
        ],
    )

    write_sql(
        "04_seed_publishers.sql",
        [
            f"INSERT INTO publishers (rawg_id, name) VALUES ({pid}, '{sanitize(name)}') ON CONFLICT (rawg_id) DO NOTHING;"
            for pid, name in all_publishers.items()
        ],
    )

    write_sql(
        "05_seed_tags.sql",
        [
            f"INSERT INTO tags (rawg_id, name) VALUES ({tid}, '{sanitize(name)}') ON CONFLICT (rawg_id) DO NOTHING;"
            for tid, name in all_tags.items()
        ],
    )

    write_sql("06_seed_games.sql", games_sql)

    write_sql(
        "07_seed_game_developers.sql",
        [
            f"INSERT INTO game_developers (game_id, developer_id)\n"
            f"SELECT g.id, d.id FROM games g, developers d\n"
            f"WHERE g.rawg_id = {gid} AND d.rawg_id = {did};"
            for gid, did in game_dev_rel
        ],
    )

    write_sql(
        "08_seed_game_publishers.sql",
        [
            f"INSERT INTO game_publishers (game_id, publisher_id)\n"
            f"SELECT g.id, p.id FROM games g, publishers p\n"
            f"WHERE g.rawg_id = {gid} AND p.rawg_id = {pid};"
            for gid, pid in game_pub_rel
        ],
    )

    write_sql(
        "09_seed_game_genres.sql",
        [
            f"INSERT INTO game_genres (game_id, genre_id)\n"
            f"SELECT g.id, gen.id FROM games g, genres gen\n"
            f"WHERE g.rawg_id = {gid} AND gen.rawg_id = {genid};"
            for gid, genid in game_genre_rel
        ],
    )

    write_sql(
        "10_seed_game_platforms.sql",
        [
            f"INSERT INTO game_platforms (game_id, platform_id)\n"
            f"SELECT g.id, p.id FROM games g, platforms p\n"
            f"WHERE g.rawg_id = {gid} AND p.rawg_id = {pid};"
            for gid, pid in game_platform_rel
        ],
    )

    write_sql(
        "11_seed_game_tags.sql",
        [
            f"INSERT INTO game_tags (game_id, tag_id)\n"
            f"SELECT g.id, t.id FROM games g, tags t\n"
            f"WHERE g.rawg_id = {gid} AND t.rawg_id = {tid};"
            for gid, tid in game_tag_rel
        ],
    )

    print(f"\nDone. {trailers_found}/{TRAILER_LIMIT} trailers downloaded.")


if __name__ == "__main__":
    main()
