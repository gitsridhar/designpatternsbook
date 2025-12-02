package command.java;

public class Peel implements Action {
    private String vegetable;

    public Peel(String vegetable) {
        this.vegetable = vegetable;
    }

    @Override
    public void Doit() {
        System.out.println("Peeling the " + vegetable);
    }
}
