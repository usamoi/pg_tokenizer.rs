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

    url = "https://huggingface.co/google-bert/bert-base-chinese/resolve/main/tokenizer.json"
    config = download_tokenizer(url)

    connection = psycopg.connect(args.db_url)
    model_name = 'model1'
    with connection.cursor() as cursor:
        cursor.execute("SELECT create_huggingface_model(%s, %s)", (model_name, config))
        cursor.execute("""SELECT tokenizer_catalog.create_tokenizer('tokenizer1', 'model = "model1"')""")
        cursor.execute("""SELECT tokenizer_catalog.tokenize('我们中出了一个叛徒', 'tokenizer1');""")
        result = cursor.fetchone()[0]
        expected = [2769, 812, 704, 1139, 749, 671, 702, 1361, 2530]
        if result != expected:
            print(f"Expected {expected}, but got {result}")
