import requests
import threading

def send_request(url):
    # Send a GET request to the URL
    response = requests.get(url)
    # Print the response status code
    print("Response:", response.status_code)

def main():
    # Define the URL you want to send requests to
    url = "http://127.0.0.1:8010/"

    # Create a list to hold the threads
    threads = []

    # Create 10 threads, each sending a request
    for _ in range(1000):
        thread = threading.Thread(target=send_request, args=(url,))
        threads.append(thread)
        thread.start()

    # Wait for all threads to complete
    for thread in threads:
        thread.join()

if __name__ == "__main__":
    main()