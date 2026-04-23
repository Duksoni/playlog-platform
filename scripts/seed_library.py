import os
import random
from datetime import datetime, timedelta, timezone

import psycopg2
from dotenv import load_dotenv

load_dotenv()

# Global seed for any non-user-specific randomness
GLOBAL_SEED = 7

# --- PostgreSQL Configuration (Users) ---
PG_USERS_HOST = os.getenv("DB_HOST", "localhost")
PG_USERS_PORT = os.getenv("USER_DB_PORT", "5433")
PG_USERS_DB = os.getenv("USER_DB_NAME", "users_db")
PG_USERS_USER = os.getenv("DB_USER", "playlog_user")
PG_USERS_PASS = os.getenv("USERS_DB_PASSWORD")

# --- PostgreSQL Configuration (Library) ---
PG_LIB_HOST = os.getenv("DB_HOST", "localhost")
PG_LIB_PORT = os.getenv("LIBRARY_DB_PORT", "5435")
PG_LIB_DB = os.getenv("LIBRARY_DB_NAME", "library_db")
PG_LIB_USER = os.getenv("DB_USER", "playlog_user")
PG_LIB_PASS = os.getenv("LIBRARY_DB_PASSWORD")

# Statuses that allow reviews
REVIEW_STATUSES = ["COMPLETED", "DROPPED"]
# Other statuses
OTHER_STATUSES = ["OWNED", "PLAYING", "WISHLIST"]


def fetch_users():
    print(f"Connecting to Postgres (Users) at {PG_USERS_HOST}:{PG_USERS_PORT}...")
    conn = psycopg2.connect(
        host=PG_USERS_HOST,
        port=PG_USERS_PORT,
        dbname=PG_USERS_DB,
        user=PG_USERS_USER,
        password=PG_USERS_PASS,
    )
    try:
        with conn.cursor() as cur:
            cur.execute("SELECT id FROM users ORDER BY id;")
            return [row[0] for row in cur.fetchall()]
    finally:
        conn.close()


def main():
    if not all([PG_LIB_DB, PG_LIB_PASS]):
        print("Error: LIBRARY_DB_NAME and LIBRARY_DB_PASSWORD must be set in .env")
        return

    try:
        user_ids = fetch_users()
    except Exception as e:
        print(f"Failed to fetch users: {e}")
        return

    if not user_ids:
        print("No users found. Please seed users first.")
        return

    print(f"Found {len(user_ids)} users.")

    print(f"Connecting to Postgres (Library) at {PG_LIB_HOST}:{PG_LIB_PORT}...")
    lib_conn = psycopg2.connect(
        host=PG_LIB_HOST,
        port=PG_LIB_PORT,
        dbname=PG_LIB_DB,
        user=PG_LIB_USER,
        password=PG_LIB_PASS,
    )
    lib_conn.autocommit = False

    game_ids = list(range(1, 201))
    total_entries = 0
    now = datetime.now(timezone.utc)

    print("Generating library entries...")
    try:
        with lib_conn.cursor() as cur:
            for user_id_uuid in user_ids:
                # MANDATORY: Seed based on User ID to ensure consistency with seed_reviews.py
                user_seed = (
                    int(user_id_uuid.replace("-", ""), 16)
                    if isinstance(user_id_uuid, str)
                    else user_id_uuid.int
                )
                random.seed(user_seed + GLOBAL_SEED)

                # 1. Pick games that WILL be reviewed (matching seed_reviews.py)
                num_reviews = random.randint(50, 100)
                review_games = random.sample(game_ids, num_reviews)

                # 2. Pick additional games for other statuses
                num_extra = random.randint(20, 50)
                remaining_games = [g for g in game_ids if g not in review_games]
                extra_games = random.sample(remaining_games, num_extra)

                # Insert Review Games (Completed/Dropped)
                for game_id in review_games:
                    status = random.choice(REVIEW_STATUSES)
                    random_days = random.randint(0, 180)
                    random_seconds = random.randint(0, 86400)
                    added_at = now - timedelta(days=random_days, seconds=random_seconds)

                    cur.execute(
                        "INSERT INTO user_games (user_id, game_id, status, added_at, last_updated) VALUES (%s, %s, %s, %s, %s) ON CONFLICT DO NOTHING;",
                        (user_id_uuid, game_id, status, added_at, added_at),
                    )
                    total_entries += 1

                # Insert Extra Games (Owned/Playing/Wishlist)
                for game_id in extra_games:
                    status = random.choice(OTHER_STATUSES)
                    random_days = random.randint(0, 180)
                    random_seconds = random.randint(0, 86400)
                    added_at = now - timedelta(days=random_days, seconds=random_seconds)

                    cur.execute(
                        "INSERT INTO user_games (user_id, game_id, status, added_at, last_updated) VALUES (%s, %s, %s, %s, %s) ON CONFLICT DO NOTHING;",
                        (user_id_uuid, game_id, status, added_at, added_at),
                    )
                    total_entries += 1

                if total_entries % 1000 == 0:
                    print(f"  -> {total_entries} library entries processed...")

            lib_conn.commit()
            print(
                f"\nSuccessfully seeded {total_entries} library entries into PostgreSQL."
            )

    except Exception as e:
        lib_conn.rollback()
        print(f"Error during seeding: {e}")
    finally:
        lib_conn.close()


if __name__ == "__main__":
    main()
