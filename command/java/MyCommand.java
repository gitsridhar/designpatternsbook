package command.java;

public class MyCommand {
    public static void main(String[] args) {

        Waiter waiter = new Waiter(
                new Peel("potato"),
                new Peel("carrot")
        );
        waiter.executeActions();

        Customer customer = new Customer();
        customer.orderFood();
        customer.makePayment();

        CustomerInteraction interaction1 = new CustomerInteraction(customer, "order");
        CustomerInteraction interaction2 = new CustomerInteraction(customer, "payment");
        Waiter waiter2 = new Waiter(interaction1, interaction2);
        waiter2.executeActions();
    }
}
