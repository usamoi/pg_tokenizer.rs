import requests
import psycopg
import argparse

# Download tokenizer from Hugging Face model hub
def download_tokenizer(url):
    response = requests.get(url)
    response.raise_for_status()
    return response.content.decode("utf-8")

def call_create_huggingface_model(connection, model_name, text):
    try:
        with connection.cursor() as cursor:
            # Execute the function
            cursor.execute("SELECT create_huggingface_model(%s, %s)", (model_name, text))
            result = cursor.fetchone()
            print(f"Result from create_huggingface_model: {result}")
    except psycopg3.Error as e:
        print(f"An error occurred while calling the function: {e}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Download tokenizer and call database function.')
    parser.add_argument('--db_url', type=str, required=True,
                        help='Database connection URL (e.g., postgresql://user:password@host:port/dbname)')
    args = parser.parse_args()

    url = "https://huggingface.co/google-bert/bert-base-uncased/resolve/main/tokenizer.json"
    config = download_tokenizer(url)

    connection = psycopg.connect(args.db_url)
    model_name = 'model1'
    with connection.cursor() as cursor:
        cursor.execute("SELECT create_huggingface_model(%s, %s)", (model_name, config))
        cursor.execute("""SELECT tokenizer_catalog.create_tokenizer('tokenizer1', 'model = "model1"')""")
        cursor.execute("""SELECT tokenizer_catalog.tokenize('PostgreSQL is a powerful, open-source object-relational database system. It has over 15 years of active development.', 'tokenizer1');""")
        result = cursor.fetchone()[0]
        expected = [2695, 17603, 2015, 4160, 2140, 2003, 1037, 3928, 1010, 2330, 1011, 3120, 4874, 1011, 28771, 7809, 2291, 1012, 2009, 2038, 2058, 2321, 2086, 1997, 3161, 2458, 1012]
        if result != expected:
            print(f"Expected {expected}, but got {result}")
