import java.io.IOException;
import java.util.Scanner;

public class Main {

    public static void main(String[] args) {
        int a,b,c,d,e;
        Scanner input = new Scanner(System.in);
        a = input.nextInt();
        b = input.nextInt();
        c = input.nextInt();
        d = input.nextInt();
        e = (a * b - c * d);
        System.out.printf("DIFERENCA = %d\n", e);
    }
}
