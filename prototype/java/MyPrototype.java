package prototype.java;

public class MyPrototype {
    public static void main(String[] args) {
        PrototypeFactory pf = new PrototypeFactory();
        Prototype pt = pf.clone1();
        Prototype pt1 = pt.clone();
        System.out.println(pt1);

        Prototype ptalt = pf.clone2();
        Prototype ptalt1 = ptalt.clone();
        System.out.println(ptalt1);
    }
}
