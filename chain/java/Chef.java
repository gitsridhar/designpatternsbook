package chain.java;

public interface Chef {
    Chef setNextChef(Chef chef);
    void cook(String dish);
}
