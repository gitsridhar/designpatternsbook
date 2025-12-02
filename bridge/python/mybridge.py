import queue
import threading
import time

class Consume:
    def __init__(self, queue):
        self.queue = queue

    def start_consuming(self):
        while True:
            message = self.queue.get()
            if message is None:
                break
            self.process_message(message)

    def process_message(self, message):
        print(f"Processing message: {message}")
        
class ConsumeJuice(Consume):
    def process_message(self, message):
        print(f"Juice Consumer processing message: {message}")
        
class ConsumeJelly(Consume):
    def process_message(self, message):
        print(f"Jelly Consumer processing message: {message}")
        
class Life:
    consume = Consume(queue)
    
    def __init__(self, consume):
        self.consume = consume
    
    def start(self):
        self.consume.start_consuming()
        
class MyLife(Life):
    consume = ConsumeJuice(queue.Queue())
    
    def __init__(self, consume):
        super().__init__(consume)
        
    def start(self):
        print("MyLife starting with Juice Consumer")
        super().start()
    
if __name__ == "__main__":
 
    msg_queue = queue.Queue()

    consumer = ConsumeJuice(msg_queue)
    consumer_thread = threading.Thread(target=consumer.start_consuming)
    consumer_thread.start()

    for i in range(5):
        msg_queue.put(f"Message {i}")
        time.sleep(1)

    msg_queue.put(None)  # Signal the consumer to stop
    consumer_thread.join()
    
    another_consumer = ConsumeJelly(queue.Queue())
    my_life = MyLife(another_consumer)
    my_life_thread = threading.Thread(target=my_life.start)
    my_life_thread.start()
    for i in range(5, 10):
        another_consumer.queue.put(f"Jelly Message {i}")
        time.sleep(1)
    another_consumer.queue.put(None)  # Signal the consumer to stop
    my_life_thread.join()
    print("All consumers have finished processing.")
    
    