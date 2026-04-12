import os
import random
import uuid
from datetime import datetime, timedelta, timezone

import psycopg2
from bson.binary import Binary
from dotenv import load_dotenv
from pymongo import MongoClient

load_dotenv()

# Global seed for consistency
GLOBAL_SEED = 7

# --- PostgreSQL Configuration ---
PG_HOST = os.getenv("DB_HOST", "localhost")
PG_PORT = os.getenv("USER_DB_PORT", "5433")
PG_DB = os.getenv("USER_DB_NAME", "users_db")
PG_USER = os.getenv("DB_USER", "playlog_user")
PG_PASS = os.getenv("USERS_DB_PASSWORD")

# --- MongoDB Configuration ---
MONGO_URL = os.getenv("REVIEW_MONGODB_URI")
MONGO_DB_NAME = "reviews_db"
REVIEWS_COLLECTION = "reviews"
COMMENTS_COLLECTION = "comments"
REPORTS_COLLECTION = "reports"

# --- Report Reasons ---

LEGIT_REASONS = [
    "Harassment or bullying",
    "Inappropriate language",
    "Hate speech",
    "Spam or misleading",
    "Encouraging violence",
    "Self-harm",
]

NONSENSE_REASONS = [
    "This user is way better than me at this game, it's unfair.",
    "I accidentally clicked this while trying to eat pizza.",
    "Testing if the report button actually does anything. Hello mods!",
    "His profile picture is staring at me weirdly.",
    "I don't like his opinion on the final boss mechanics.",
    "He said my favorite game is 'just okay'. Unacceptable.",
    "Spelling mistake on line 3 of his 500-word essay.",
    "Reported for being too cool.",
]

# From seed_comments.py to identify "toxic" content
TOXIC_TEMPLATES = [
    "You clearly have no taste in games. This review is trash.",
    "Imagine actually liking this garbage. Go touch grass.",
    "This review is so stupid, did you even play the game?",
    "Trash take from a trash player. Delete this.",
    "You're an idiot if you think the mechanics are good.",
    "Lmao, git gud scrub. You probably couldn't even get past the first level.",
    "Worst review I've ever read. You should be banned for this.",
    "Stop posting your worthless opinions online.",
    "This game is for children, and so is your review.",
    "Literal braindead take. Go play something simple like Minecraft.",
    "I hope the developers see this and laugh at how wrong you are.",
    "How much did they pay you to write this fake positive review?",
    "You are what's wrong with the gaming community.",
    "Nobody cares what you think. Uninstall the game and your life.",
    "Stop lying, the optimization is fine. Your PC is just a potato.",
    "Absolute clown behavior. Honk honk.",
    "Your brain is as empty as this game's open world.",
    "Shut up and stop crying about the difficulty.",
    "Just refilled my coffee with your salty tears. Delicious.",
    "Garbage game, garbage review, garbage user.",
]


def to_uuid_bytes(u_id):
    if isinstance(u_id, uuid.UUID):
        return u_id.bytes
    if isinstance(u_id, str):
        return uuid.UUID(u_id).bytes
    return bytes(u_id)


def fetch_users():
    print(f"Connecting to Postgres at {PG_HOST}:{PG_PORT}...")
    conn = psycopg2.connect(
        host=PG_HOST, port=PG_PORT, dbname=PG_DB, user=PG_USER, password=PG_PASS
    )
    try:
        with conn.cursor() as cur:
            cur.execute(
                "SELECT id, username FROM users u JOIN user_profiles up ON u.id = up.user_id ORDER BY id;"
            )
            return cur.fetchall()
    finally:
        conn.close()


def main():
    if not MONGO_URL:
        print("Error: REVIEW_MONGODB_URI not set in .env")
        return

    try:
        users = fetch_users()
    except Exception as e:
        print(f"Failed to fetch users: {e}")
        return

    if not users:
        print("No users found. Please seed users first.")
        return

    print(f"Found {len(users)} users.")

    print("Connecting to MongoDB...")
    mongo_client = MongoClient(MONGO_URL)
    db = mongo_client[MONGO_DB_NAME]

    reports_collection = db[REPORTS_COLLECTION]
    # Match indexing from Rust service if needed
    reports_collection.create_index([("status", 1)])
    reports_collection.create_index([("created_at", -1)])

    now = datetime.now(timezone.utc)

    # 1. Identify "Toxic" Comments
    print("Finding reportable comments...")
    toxic_comments = list(
        db[COMMENTS_COLLECTION].find(
            {"text": {"$in": TOXIC_TEMPLATES}, "deleted": False},
            {"_id": 1, "user_id": 1, "created_at": 1},
        )
    )
    print(f"  -> Found {len(toxic_comments)} potentially toxic comments.")

    # 2. Identify "NOT_RECOMMENDED" Reviews (often more heated)
    print("Finding reportable reviews...")
    toxic_reviews = list(
        db[REVIEWS_COLLECTION].find(
            {"rating": "NOT_RECOMMENDED", "deleted": False},
            {"_id": 1, "user_id": 1, "created_at": 1},
        )
    )
    print(f"  -> Found {len(toxic_reviews)} 'NOT_RECOMMENDED' reviews.")

    # 3. Identify some random content for "nonsense" reports
    random_comments = list(
        db[COMMENTS_COLLECTION].aggregate(
            [{"$match": {"deleted": False}}, {"$sample": {"size": 50}}]
        )
    )
    random_reviews = list(
        db[REVIEWS_COLLECTION].aggregate(
            [{"$match": {"deleted": False}}, {"$sample": {"size": 50}}]
        )
    )

    total_reports = 0
    batch = []

    def create_report(target, target_type, is_legit):
        nonlocal total_reports
        target_id = target["_id"]
        target_author_bytes = target["user_id"]
        target_created_at = target["created_at"].replace(tzinfo=timezone.utc)

        # Pick a reporter who isn't the author
        valid_reporters = [
            u for u in users if to_uuid_bytes(u[0]) != target_author_bytes
        ]
        if not valid_reporters:
            return

        reporter_id_val, reporter_username = random.choice(valid_reporters)

        # Deterministic seed for this specific report
        random.seed(int(str(target_id)[-8:], 16) + GLOBAL_SEED)

        reason = (
            random.choice(LEGIT_REASONS)
            if is_legit
            else random.choice(NONSENSE_REASONS)
        )

        start_date = max(target_created_at, now - timedelta(days=30))
        delta = now - start_date
        report_date = (
            start_date
            + timedelta(seconds=random.randint(0, int(delta.total_seconds())))
            if delta.total_seconds() > 0
            else now
        )

        batch.append(
            {
                "target_type": target_type,
                "target_id": target_id,
                "reporter_id": Binary(to_uuid_bytes(reporter_id_val), 0),
                "reporter_username": reporter_username,
                "reason": reason,
                "status": "PENDING",
                "created_at": report_date,
                "version": 0,
            }
        )
        total_reports += 1

    # Legit reports on toxic comments (80% chance for each toxic comment)
    print("Generating legitimate reports on comments...")
    for comment in toxic_comments:
        if random.random() < 0.8:
            create_report(comment, "COMMENT", is_legit=True)

    # Legit reports on negative reviews (20% chance)
    print("Generating legitimate reports on reviews...")
    for review in toxic_reviews:
        if random.random() < 0.2:
            create_report(review, "REVIEW", is_legit=True)

    # Nonsense reports (random sample)
    print("Generating nonsensical reports...")
    for comment in random_comments:
        if random.random() < 0.3:
            create_report(comment, "COMMENT", is_legit=False)
    for review in random_reviews:
        if random.random() < 0.3:
            create_report(review, "REVIEW", is_legit=False)

    if batch:
        reports_collection.insert_many(batch)
        print(f"\nSuccessfully seeded {total_reports} reports into MongoDB.")
    else:
        print("\nNo reports were generated.")

    mongo_client.close()


if __name__ == "__main__":
    main()
