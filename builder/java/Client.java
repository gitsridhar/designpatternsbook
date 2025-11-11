package builder.java;

public class Client {
    private Director director;
    public Client(Director director) {
        this.director = director;
    }
    
    public void Doit() {
        AppleConsumer ac = new AppleConsumer();
        director.set_consumer(ac);
        director.consume_fruit();

        Fruit fruit = ac.getFruit();
        fruit.getUtensils();
    }

    public static void main(String[] args) {
        Director director = new Director();
        Client client = new Client(director);
        client.Doit();
    }
}
