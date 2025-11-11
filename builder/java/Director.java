package builder.java;

public class Director {
    private AppleConsumer appleconsumer;

    void set_consumer(AppleConsumer consumer) {
        this.appleconsumer = consumer;
    }

    void consume_fruit() {
        appleconsumer.Peeler();
        appleconsumer.Knife();
    }
}
