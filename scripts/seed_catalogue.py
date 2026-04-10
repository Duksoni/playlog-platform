import os
import psycopg2
from dotenv import load_dotenv
from pathlib import Path

load_dotenv()

DB_HOST = os.getenv("DB_HOST", "localhost")
DB_PORT = os.getenv("CATALOGUE_DB_PORT", "5434")
DB_NAME = os.getenv("CATALOGUE_DB_NAME")
DB_USER = os.getenv("DB_USER")
DB_PASSWORD = os.getenv("CATALOGUE_DB_PASSWORD")

SQL_ROOT = Path("sql_seed")


def main():
    if not all([DB_NAME, DB_USER, DB_PASSWORD]):
        raise ValueError(
            "CATALOGUE_DB_NAME, DB_USER, and DB_PASSWORD must be set in your .env file."
        )

    sql_files = sorted(SQL_ROOT.glob("*.sql"))

    if not sql_files:
        print(f"No SQL files found in {SQL_ROOT}/")
        return

    print(f"Found {len(sql_files)} SQL files to execute:\n")
    for f in sql_files:
        print(f"  {f.name}")

    print(f"\nConnecting to {DB_USER}@{DB_HOST}:{DB_PORT}/{DB_NAME}...")

    conn = psycopg2.connect(
        host=DB_HOST,
        port=DB_PORT,
        dbname=DB_NAME,
        user=DB_USER,
        password=DB_PASSWORD,
    )
    conn.autocommit = False

    try:
        with conn.cursor() as cur:
            for sql_file in sql_files:
                print(f"\nExecuting {sql_file.name}...")
                sql = sql_file.read_text(encoding="utf-8")
                cur.execute(sql)
                print(f"  -> OK ({cur.rowcount} rows affected)")

        conn.commit()
        print("\nAll seed files executed successfully.")

    except Exception as e:
        conn.rollback()
        print(f"\nError: {e}")
        print("Transaction rolled back. No changes were committed.")
        raise

    finally:
        conn.close()


if __name__ == "__main__":
    main()
