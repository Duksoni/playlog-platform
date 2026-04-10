import os
import random
import psycopg2
from argon2 import PasswordHasher
from dotenv import load_dotenv
from datetime import date, datetime, timedelta, timezone

load_dotenv()

random.seed(7)

# Configuration
N = 500
DB_HOST = os.getenv("DB_HOST", "localhost")
DB_PORT = os.getenv("USER_DB_PORT", "5433")  # Default to 5433 for users_db
DB_NAME = os.getenv("USER_DB_NAME", "users_db")
DB_USER = os.getenv("DB_USER", "playlog_user")
DB_PASSWORD = os.getenv("USERS_DB_PASSWORD") # From compose.yaml

FIRST_NAMES = [
    "James", "Mary", "Robert", "Patricia", "John", "Jennifer", "Michael", "Linda",
    "David", "Elizabeth", "William", "Barbara", "Richard", "Susan", "Joseph",
    "Jessica", "Thomas", "Sarah", "Christopher", "Karen", "Charles", "Nancy",
    "Daniel", "Lisa", "Matthew", "Betty", "Anthony", "Margaret", "Mark", "Sandra"
]

LAST_NAMES = [
    "Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis",
    "Rodriguez", "Martinez", "Hernandez", "Lopez", "Gonzalez", "Wilson", "Anderson",
    "Thomas", "Taylor", "Moore", "Jackson", "Martin", "Lee", "Perez", "Thompson",
    "White", "Harris", "Sanchez", "Clark", "Ramirez", "Lewis", "Robinson"
]

DOMAINS = ["example.com", "test.com", "gmail.com", "outlook.com", "yahoo.com"]

def generate_random_birthdate():
    start_date = date(1970, 1, 1)
    end_date = date(2010, 1, 1)
    days_between = (end_date - start_date).days
    random_days = random.randrange(days_between)
    return start_date + timedelta(days=random_days)

def generate_account_creation_date():
    now = datetime.now(timezone.utc)
    random_days = random.randint(210, 365)
    random_seconds = random.randint(0, 86400)
    return now - timedelta(days=random_days, seconds=random_seconds)

def main():
    if not DB_PASSWORD:
        print("Warning: USERS_DB_PASSWORD not set in environment. Connection might fail.")

    ph = PasswordHasher()
    default_password_hash = ph.hash("password123")
    
    print(f"Connecting to {DB_NAME} on {DB_HOST}:{DB_PORT}...")
    try:
        conn = psycopg2.connect(
            host=DB_HOST,
            port=DB_PORT,
            dbname=DB_NAME,
            user=DB_USER,
            password=DB_PASSWORD
        )
        conn.autocommit = False
        cur = conn.cursor()

        # Fetch role IDs
        cur.execute("SELECT id, name FROM roles;")
        roles = {name: id for id, name in cur.fetchall()}
        
        if not roles:
            print("Error: No roles found in the database. Please run migrations first.")
            return

        print(f"Generating {N} users...")
        
        users_inserted = 0
        usernames_set = set()
        emails_set = set()

        for i in range(N):
            first = random.choice(FIRST_NAMES)
            last = random.choice(LAST_NAMES)
            
            # Ensure uniqueness for username and email
            base_username = f"{first.lower()}{last.lower()}"
            username = base_username
            attempt = 1
            while username in usernames_set:
                username = f"{base_username}{random.randint(1, 9999)}"
                attempt += 1
            usernames_set.add(username)

            email = f"{username}@{random.choice(DOMAINS)}"
            while email in emails_set:
                email = f"{username}{random.randint(1, 999)}@{random.choice(DOMAINS)}"
            emails_set.add(email)

            # Insert User
            cur.execute(
                "INSERT INTO users (username, email, password, account_status) VALUES (%s, %s, %s, 'ACTIVE') RETURNING id;",
                (username, email, default_password_hash)
            )
            user_id = cur.fetchone()[0]

            # Insert Profile with realistic created_at date
            birthdate = generate_random_birthdate()
            created_at = generate_account_creation_date()
            cur.execute(
                "INSERT INTO user_profiles (user_id, first_name, last_name, birthdate, created_at) VALUES (%s, %s, %s, %s, %s);",
                (user_id, first, last, birthdate, created_at)
            )

            # Assign Role (95% USER, 4% MODERATOR, 1% ADMIN)
            r = random.random()
            if r < 0.01 and "ADMIN" in roles:
                role_name = "ADMIN"
            elif r < 0.05 and "MODERATOR" in roles:
                role_name = "MODERATOR"
            else:
                role_name = "USER"
            
            cur.execute(
                "INSERT INTO user_roles (user_id, role_id) VALUES (%s, %s);",
                (user_id, roles[role_name])
            )

            users_inserted += 1
            if users_inserted % 50 == 0:
                print(f"  -> {users_inserted} users processed...")

        conn.commit()
        print(f"\nSuccessfully seeded {users_inserted} users into the database.")

    except Exception as e:
        if 'conn' in locals():
            conn.rollback()
        print(f"Error during seeding: {e}")
    finally:
        if 'conn' in locals():
            conn.close()

if __name__ == "__main__":
    main()
