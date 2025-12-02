package command.java;

public class CustomerInteraction implements Action {
    private Customer customer;
    private String interactionType;

    public CustomerInteraction(Customer customer, String interactionType) {
        this.customer = customer;
        this.interactionType = interactionType;
    }

    @Override
    public void Doit() {
        if (interactionType.equals("order")) {
            customer.orderFood();
            System.out.println("Customer is ordering food.");
        } else if (interactionType.equals("payment")) {
            customer.makePayment();
            System.out.println("Customer is making a payment.");
        } else {
            System.out.println("Unknown interaction type.");
        }
    }
}
