package command.java;

public class Waiter {
    private Action action1;
    private Action action2;

    public Waiter(Action action1, Action action2) {
        this.action1 = action1;
        this.action2 = action2;
    }

    public void executeActions() {
        if (action1 != null)
            action1.Doit();
        if (action2 != null)
            action2.Doit();
    }
}
