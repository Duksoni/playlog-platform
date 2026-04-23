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

# --- Comment Templates ---

CONSTRUCTIVE_TEMPLATES = [
    "I totally agree with this! Spot on.",
    "Interesting take, but I found the combat a bit clunky.",
    "Can't wait to try this one out, looks amazing.",
    "The soundtrack is definitely the highlight for me too.",
    "How long did it take you to beat it?",
    "Great review, very helpful for my next purchase!",
    "I had the same experience with the final boss, super frustrating.",
    "Does this game have a co-op mode? Looking for something to play with friends.",
    "The graphics are a bit dated, but the art style saves it.",
    "One of the best games I've played this year, hands down.",
    "I think you're being a bit too harsh on the story.",
    "Is the DLC worth getting as well?",
    "This game was a masterpiece of storytelling.",
    "I couldn't get into it, felt too grindy for me.",
    "The atmosphere is just unmatched.",
    "Anyone else having performance issues on PC?",
    "This reminds me so much of the older titles in the series.",
    "Absolutely breathtaking visuals.",
    "I've been playing this for 50 hours and still finding new things.",
    "Thanks for the detailed breakdown!",
]

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
                "SELECT id, username, up.created_at FROM users u JOIN user_profiles up ON u.id = up.user_id ORDER BY id;"
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

    print("Fetching existing reviews...")
    # Fetch reviews including user_id to prevent self-commenting
    all_reviews = list(
        db[REVIEWS_COLLECTION].find(
            {"deleted": False}, {"_id": 1, "created_at": 1, "user_id": 1}
        )
    )

    comments_collection = db[COMMENTS_COLLECTION]
    comments_collection.create_index([("target_type", 1), ("target_id", 1)])
    comments_collection.create_index([("user_id", 1)])

    total_comments = 0
    now = datetime.now(timezone.utc)

    all_templates = CONSTRUCTIVE_TEMPLATES + TOXIC_TEMPLATES
    weights = [0.7] * len(CONSTRUCTIVE_TEMPLATES) + [0.3] * len(TOXIC_TEMPLATES)

    # 1. Review Comments
    # Objective: 50% of reviews get 2-5 comments
    print("Generating review comments (50% coverage, 2-5 per review)...")
    commented_reviews = random.sample(all_reviews, int(len(all_reviews) * 0.5))

    for review in commented_reviews:
        review_id_str = str(review["_id"])
        review_author_bytes = review["user_id"]  # Binary from Mongo
        review_created_at = review["created_at"].replace(tzinfo=timezone.utc)

        num_to_add = random.randint(2, 5)
        # Filter pool to prevent self-commenting
        valid_commenters = [
            u for u in users if to_uuid_bytes(u[0]) != review_author_bytes
        ]

        batch = []
        for user_id_val, username, user_created_at in random.sample(
            valid_commenters, num_to_add
        ):
            u_seed = int(to_uuid_bytes(user_id_val).hex()[:8], 16)
            random.seed(u_seed + GLOBAL_SEED + int(review_id_str[-6:], 16))

            start_date = max(user_created_at, review_created_at)
            delta = now - start_date

            if delta.total_seconds() > 0:
                random_seconds = random.randint(0, int(delta.total_seconds()))
                comment_date = start_date + timedelta(seconds=random_seconds)
                template = random.choices(all_templates, weights=weights)[0]

                batch.append(
                    {
                        "target_type": "REVIEW",
                        "target_id": review_id_str,
                        "user_id": Binary(to_uuid_bytes(user_id_val), 0),
                        "username": username,
                        "text": template,
                        "created_at": comment_date,
                        "updated_at": comment_date,
                        "version": 0,
                        "deleted": False,
                    }
                )

        if batch:
            comments_collection.insert_many(batch)
            total_comments += len(batch)
            if total_comments % 5000 < 5:  # Log progress occasionally
                print(f"  -> {total_comments} review comments processed...")

    # 2. Game Comments
    # Objective: At least 50 for each game (1-200)
    print(
        f"Generated {total_comments} review comments. Now generating 50 comments per game..."
    )

    for game_id in range(1, 201):
        batch = []
        # Randomly sample 50 users for this game
        for user_id_val, username, user_created_at in random.sample(users, 50):
            # Seed based on user + game for determinism
            random.seed(
                int(to_uuid_bytes(user_id_val).hex()[:8], 16) + game_id + GLOBAL_SEED
            )

            start_date = user_created_at
            delta = now - start_date

            if delta.total_seconds() > 0:
                random_seconds = random.randint(0, int(delta.total_seconds()))
                comment_date = start_date + timedelta(seconds=random_seconds)
                template = random.choices(all_templates, weights=weights)[0]

                batch.append(
                    {
                        "target_type": "GAME",
                        "target_id": str(game_id),
                        "user_id": Binary(to_uuid_bytes(user_id_val), 0),
                        "username": username,
                        "text": template,
                        "created_at": comment_date,
                        "updated_at": comment_date,
                        "version": 0,
                        "deleted": False,
                    }
                )

        if batch:
            comments_collection.insert_many(batch)
            total_comments += len(batch)
            if game_id % 20 == 0:
                print(
                    f"  -> {total_comments} total comments processed (Game {game_id}/200)..."
                )

    print(f"\nSuccessfully seeded {total_comments} total comments into MongoDB.")
    mongo_client.close()


if __name__ == "__main__":
    main()
