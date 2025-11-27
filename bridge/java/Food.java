package bridge.java;

public abstract class Food {
    protected Pan pan;

    public Food(Pan pan) {
        this.pan = pan;
    }

    public abstract void prepare();
}
