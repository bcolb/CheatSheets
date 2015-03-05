/**
 * RecursiveExponentiation.java
 */
import java.util.Scanner;
/**
 * An implementation of recursive exponentiation in logN time.
 */
public class RecursiveExponentiation {

    public static long pow(long base, int exponent) {
	if(exponent==0) return 1;
	if(exponent % 2 == 0) {
	    return pow(base * base, exponent / 2);
	} else {
	    return pow(base * base, exponent / 2) * base;
	}
    }

    public static void main(String[] args) {
	// Expects to lines, the first a long (the base)
	// the second an int (the exponent)
	Scanner scan = new Scanner(System.in);
	long base = Long.parseLong(scan.nextLine());
	int exponent = Integer.parseInt(scan.nextLine());
	long result = pow(base, exponent);
	System.out.println("result: " + result);
    }
}
