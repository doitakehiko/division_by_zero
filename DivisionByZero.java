public class DivisionByZero {
	public static void main( String args[] ) {
		int x = 1 / 0;
	}
}
/*
javac .\DivisionByZero.java
java  DivisionByZero
Exception in thread "main" java.lang.ArithmeticException: / by zero
        at DivisionByZero.main(DivisionByZero.java:3)
*/