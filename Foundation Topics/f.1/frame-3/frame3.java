public class frame3 {
    public static void valueCheck(int a, int b) {
        if (a > b) {
            System.out.println(a + " > " + b);
        } else {
            System.out.println(a + " < " + b);
        }
    }

    public static void main(String[] args) {
        valueCheck(-3, -6);
        valueCheck(2, -4);
        valueCheck(-7, 12);
    }
}