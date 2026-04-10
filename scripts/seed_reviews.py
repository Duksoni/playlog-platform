import os
import random
from datetime import datetime, timedelta, timezone

import psycopg2
from bson.binary import Binary
from dotenv import load_dotenv
from pymongo import MongoClient

load_dotenv()

# Global seed for any non-user-specific randomness
GLOBAL_SEED = 7

# --- PostgreSQL Configuration (Users) ---
PG_HOST = os.getenv("DB_HOST", "localhost")
PG_PORT = os.getenv("USER_DB_PORT", "5433")
PG_DB = os.getenv("USER_DB_NAME", "users_db")
PG_USER = os.getenv("DB_USER", "playlog_user")
PG_PASS = os.getenv("USERS_DB_PASSWORD")

# --- MongoDB Configuration (Reviews) ---
MONGO_URL = os.getenv("REVIEW_MONGODB_URI")
MONGO_DB_NAME = "reviews_db"
MONGO_COLLECTION_NAME = "reviews"

# --- Review Generation Data ---

RATINGS = ["NOT_RECOMMENDED", "OKAY", "GOOD", "HIGHLY_RECOMMENDED"]

REVIEW_TEMPLATES = {
    "HIGHLY_RECOMMENDED": {
        "openers": [
            "Absolutely loved this game!",
            "A masterpiece of the genre.",
            "Simply breathtaking.",
            "One of my favorite games of all time.",
            "I couldn't put it down.",
        ],
        "gameplay": [
            "The mechanics are incredibly tight and responsive.",
            "The gameplay loop is perfect and super addictive.",
            "Combat and exploration feel so rewarding.",
            "It provides a perfect level of challenge.",
            "Innovative systems that really push the boundaries.",
        ],
        "visuals": [
            "The art style is stunning and unique.",
            "Visuals are top-notch with amazing attention to detail.",
            "The atmosphere is thick and immersive.",
            "Technically impressive and beautiful to look at.",
            "The lighting and world design are incredible.",
        ],
        "story": [
            "The narrative is emotionally resonant and deep.",
            "The world-building is masterfully done.",
            "Characters are well-developed and memorable.",
            "The plot twists kept me on the edge of my seat.",
            "Writing is sharp and very engaging.",
        ],
        "conclusions": [
            "A must-play for everyone.",
            "10/10, would play again immediately.",
            "Go buy it right now, you won't regret it.",
            "It defines what gaming should be.",
            "An absolute gem.",
        ],
    },
    "GOOD": {
        "openers": [
            "Really enjoyed my time with this.",
            "A very solid and fun experience.",
            "Surpassed my expectations.",
            "Definitely a good game worth playing.",
            "Had a great time with this one.",
        ],
        "gameplay": [
            "Mechanics are solid and fun to engage with.",
            "The gameplay is smooth and well-paced.",
            "It offers plenty of variety and interesting choices.",
            "Systems are intuitive and easy to pick up.",
            "Good balance between fun and challenge.",
        ],
        "visuals": [
            "Looks great and runs smoothly.",
            "Solid art direction throughout.",
            "The environments are well-crafted.",
            "Visual presentation is pleasing.",
            "Good graphics that suit the game's tone.",
        ],
        "story": [
            "The story is interesting and kept me engaged.",
            "Good character arcs and decent world-building.",
            "The narrative is consistent and well-told.",
            "I liked the themes and setting.",
            "Writing is solid and fits the experience.",
        ],
        "conclusions": [
            "Highly recommended for fans of the genre.",
            "A very good investment of your time.",
            "Definitely worth picking up.",
            "Solid 4/5 experience.",
            "I'll be looking forward to more from this developer.",
        ],
    },
    "OKAY": {
        "openers": [
            "It's decent, but has some flaws.",
            "An okay experience overall.",
            "Not bad, but not great either.",
            "Had some fun, but it felt lacking in areas.",
            "It's a middle-of-the-road title.",
        ],
        "gameplay": [
            "The core mechanics are fine but get repetitive.",
            "Some systems feel a bit shallow or half-baked.",
            "Gameplay is okay but lacks a certain 'oomph'.",
            "Control issues occasionally hampered the fun.",
            "The difficulty curve is a bit uneven.",
        ],
        "visuals": [
            "Graphics are acceptable but nothing special.",
            "The art style is a bit generic.",
            "Visually fine, but some textures are low-quality.",
            "The presentation is functional.",
            "Decent visuals, but lacks identity.",
        ],
        "story": [
            "The plot is predictable but functional.",
            "Characters are a bit one-dimensional.",
            "The narrative didn't really hook me.",
            "Writing is okay, if a bit uninspired.",
            "The world-building could have been deeper.",
        ],
        "conclusions": [
            "Worth it on a deep sale.",
            "Check it out if you really like the genre.",
            "A fair way to spend a few hours.",
            "Solid but forgettable.",
            "It fills a niche, but don't expect too much.",
        ],
    },
    "NOT_RECOMMENDED": {
        "openers": [
            "Disappointing experience.",
            "Wanted to like it, but I just couldn't.",
            "Quite a letdown.",
            "I'd suggest skipping this one.",
            "Frustrating and not very fun.",
            "This game is an absolute joke.",
            "Complete waste of money and time.",
            "I can't believe they released this garbage.",
        ],
        "gameplay": [
            "The gameplay loop is tedious and boring.",
            "Clunky controls made it hard to enjoy.",
            "Bugs and technical issues were a major problem.",
            "The mechanics feel dated and uninspired.",
            "Poor design choices throughout.",
            "The developers clearly have no idea what they are doing.",
            "Mechanics are broken and the AI is braindead.",
            "It's literally unplayable in this state.",
        ],
        "visuals": [
            "Visually unappealing and messy.",
            "Technically outdated and poorly optimized.",
            "The art style feels inconsistent.",
            "Muddied textures and bland environments.",
            "Presentation lacks any real polish.",
            "Looks like something from 2005, absolutely hideous.",
            "My eyes actually hurt looking at these textures.",
            "The optimization is non-existent, constant stutters.",
        ],
        "story": [
            "The story is nonsensical and boring.",
            "Writing is weak and full of clichés.",
            "I couldn't care less about the characters.",
            "The plot fails to go anywhere interesting.",
            "World-building is practically non-existent.",
            "The writing is insulting and poorly translated.",
            "Characters are so annoying I wanted to mute the game.",
            "A toddler could write a better plot than this mess.",
        ],
        "conclusions": [
            "Avoid this for now.",
            "Not worth the price or the time.",
            "A waste of potential.",
            "There are much better games in this genre.",
            "Needs a lot of work before I can recommend it.",
            "Don't buy this trash, seriously.",
            "Refunding this immediately. Pure garbage.",
            "Worst game of the year, stay far away.",
        ],
    },
}


def generate_review_text(rating):
    pool = REVIEW_TEMPLATES[rating]

    parts = [
        random.choice(pool["openers"]),
        random.choice(pool["gameplay"]),
        random.choice(pool["visuals"]),
        random.choice(pool["story"]),
        random.choice(pool["conclusions"]),
    ]

    num_to_keep = random.randint(3, 5)
    selected_parts = random.sample(parts, num_to_keep)

    ordered_parts = []
    if parts[0] in selected_parts:
        ordered_parts.append(parts[0])
    for p in parts[1:-1]:
        if p in selected_parts:
            ordered_parts.append(p)
    if parts[-1] in selected_parts:
        ordered_parts.append(parts[-1])

    return " ".join(ordered_parts)


def fetch_users():
    print(f"Connecting to Postgres at {PG_HOST}:{PG_PORT}...")
    conn = psycopg2.connect(
        host=PG_HOST, port=PG_PORT, dbname=PG_DB, user=PG_USER, password=PG_PASS
    )
    try:
        with conn.cursor() as cur:
            cur.execute("SELECT id, username FROM users ORDER BY id;")
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
        print(f"Failed to fetch users from Postgres: {e}")
        return

    if not users:
        print("No users found in database. Please seed users first.")
        return

    print(f"Found {len(users)} users.")

    print("Connecting to MongoDB...")
    mongo_client = MongoClient(MONGO_URL)
    db = mongo_client[MONGO_DB_NAME]
    collection = db[MONGO_COLLECTION_NAME]

    game_ids = list(range(1, 201))
    total_reviews = 0
    now = datetime.now(timezone.utc)

    print("Generating reviews...")

    for user_id_uuid, username in users:
        # MANDATORY: Seed based on User ID to ensure consistency with seed_library.py
        user_seed = (
            int(user_id_uuid.replace("-", ""), 16)
            if isinstance(user_id_uuid, str)
            else user_id_uuid.int
        )
        random.seed(user_seed + GLOBAL_SEED)

        # 1. Pick games that MUST match seed_library.py (first call to sample after seed)
        num_reviews = random.randint(50, 100)
        selected_games = random.sample(game_ids, num_reviews)

        reviews_to_insert = []
        for game_id in selected_games:
            # These calls are independent and don't affect game selection matching
            rating = random.choice(RATINGS)
            text = generate_review_text(rating)

            if hasattr(user_id_uuid, "bytes"):
                user_id_bytes = user_id_uuid.bytes
            else:
                import uuid

                user_id_bytes = uuid.UUID(user_id_uuid).bytes

            # We can re-use the same timestamp generation logic
            random_days = random.randint(0, 180)
            random_seconds = random.randint(0, 86400)
            created_at = now - timedelta(days=random_days, seconds=random_seconds)

            review_doc = {
                "game_id": game_id,
                "user_id": Binary(user_id_bytes, 0),
                "username": username,
                "rating": rating,
                "text": text,
                "created_at": created_at,
                "updated_at": created_at,
                "version": 0,
                "deleted": False,
            }
            reviews_to_insert.append(review_doc)

        if reviews_to_insert:
            try:
                collection.insert_many(reviews_to_insert, ordered=False)
                total_reviews += len(reviews_to_insert)
            except Exception:
                pass

        if total_reviews % 500 == 0 or total_reviews < 500:
            print(f"  -> {total_reviews} reviews processed...")

    print(f"\nSuccessfully seeded {total_reviews} reviews into MongoDB.")
    mongo_client.close()


if __name__ == "__main__":
    main()
