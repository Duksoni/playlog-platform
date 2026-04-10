import mimetypes
import os
import subprocess
import json
from datetime import datetime, timezone
from pathlib import Path

from dotenv import load_dotenv
from minio import Minio
from pymongo import ASCENDING, MongoClient

load_dotenv()

MINIO_ENDPOINT = os.getenv("MINIO_ENDPOINT", "localhost:9000")
MINIO_ACCESS_KEY = os.getenv("MINIO_ACCESS_KEY")
MINIO_SECRET_KEY = os.getenv("MINIO_SECRET_KEY")
MINIO_BUCKET = "playlog-media"

MONGO_URL = os.getenv("MEDIA_MONGODB_URI")
MONGO_DB = "multimedia_db"
MONGO_COLLECTION = "game_media"

MEDIA_ROOT = Path("media")

if not MINIO_ACCESS_KEY or not MINIO_SECRET_KEY or not MONGO_URL:
    raise ValueError(
        "MINIO_ACCESS_KEY, MINIO_SECRET_KEY and MONGO_URL must be set in your .env file."
    )


def get_file_info(path: Path) -> dict:
    mime_type, _ = mimetypes.guess_type(path)
    return {
        "mime_type": mime_type or "application/octet-stream",
        "size_bytes": path.stat().st_size,
    }


def upload_to_minio(client: Minio, local_path: Path, object_key: str) -> None:
    info = get_file_info(local_path)
    client.fput_object(
        MINIO_BUCKET,
        object_key,
        str(local_path),
        content_type=info["mime_type"],
    )


def make_media_file(local_path: Path, object_key: str) -> dict:
    info = get_file_info(local_path)
    return {
        "object_key": object_key,
        "mime_type": info["mime_type"],
        "size_bytes": info["size_bytes"],
        "uploaded_at": datetime.now(timezone.utc),
    }


def get_video_duration_seconds(path: Path) -> float | None:
    try:
        result = subprocess.run(
            [
                "ffprobe",
                "-v",
                "error",
                "-show_entries",
                "format=duration",
                "-of",
                "json",
                str(path),
            ],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            check=True,
        )

        data = json.loads(result.stdout)
        duration = float(data["format"]["duration"])
        return round(duration, 2)

    except Exception as e:
        print(f"  -> Could not read video duration for {path.name}: {e}")
        return None


def make_video_file(local_path: Path, object_key: str) -> dict:
    doc = make_media_file(local_path, object_key)
    doc["duration_seconds"] = get_video_duration_seconds(local_path)
    return doc


def main():
    game_dirs = sorted(
        [d for d in MEDIA_ROOT.iterdir() if d.is_dir()],
        key=lambda d: int(d.name),
    )

    if not game_dirs:
        print(f"No game directories found in {MEDIA_ROOT}/")
        return

    print(f"Found {len(game_dirs)} game directories\n")

    print("Connecting to MinIO...")
    minio_client = Minio(
        MINIO_ENDPOINT,
        access_key=MINIO_ACCESS_KEY,
        secret_key=MINIO_SECRET_KEY,
        secure=False,
    )

    if not minio_client.bucket_exists(MINIO_BUCKET):
        minio_client.make_bucket(MINIO_BUCKET)
        print(f"  -> Created bucket '{MINIO_BUCKET}'")
    else:
        print(f"  -> Bucket '{MINIO_BUCKET}' already exists")

    print("Connecting to MongoDB...")
    mongo_client = MongoClient(MONGO_URL)
    collection = mongo_client[MONGO_DB][MONGO_COLLECTION]
    collection.create_index([("game_id", ASCENDING)], unique=True)
    print(f"  -> Connected to {MONGO_DB}.{MONGO_COLLECTION}\n")

    inserted = 0
    skipped = 0

    for game_dir in game_dirs:
        game_id = int(game_dir.name)
        prefix = f"games/{game_id}"

        print(f"[{game_id}] Processing {game_dir.name}/")

        cover = None
        screenshots = []
        trailer = None

        cover_path = game_dir / "cover.jpg"
        if cover_path.exists():
            object_key = f"{prefix}/cover.jpg"
            upload_to_minio(minio_client, cover_path, object_key)
            cover = make_media_file(cover_path, object_key)
            print("  -> cover.jpg uploaded")

        for i in range(1, 4):
            shot_path = game_dir / f"screenshot_{i}.jpg"
            if shot_path.exists():
                object_key = f"{prefix}/screenshot_{i}.jpg"
                upload_to_minio(minio_client, shot_path, object_key)
                screenshots.append(make_media_file(shot_path, object_key))
                print(f"  -> screenshot_{i}.jpg uploaded")

        trailer_path = game_dir / "trailer.mp4"
        if trailer_path.exists():
            object_key = f"{prefix}/trailer.mp4"
            upload_to_minio(minio_client, trailer_path, object_key)
            trailer = make_video_file(trailer_path, object_key)
            print("  -> trailer.mp4 uploaded")

        document = {
            "game_id": game_id,
            "cover": cover,
            "screenshots": screenshots,
            "trailer": trailer,
            "version": 0,
        }

        try:
            collection.insert_one(document)
            inserted += 1
            print("  -> MongoDB document inserted")
        except Exception as e:
            print(f"  -> Skipped (already exists or error): {e}")
            skipped += 1

    print(f"\nDone. {inserted} inserted, {skipped} skipped.")
    mongo_client.close()


if __name__ == "__main__":
    main()
