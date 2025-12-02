package memento.java;

public class MyMemento {
   public static void main(String[] args) {
       Waiter waiter = new Waiter("Initial State");
       Chef chef = new Chef(waiter);

       System.out.println("Current State: " + waiter.getState());
       chef.backup();

       waiter = new Waiter("State After Change");
       System.out.println("Changed State: " + waiter.getState());

       chef.undo();
       System.out.println("Restored State: " + waiter.getState());
   } 
}
