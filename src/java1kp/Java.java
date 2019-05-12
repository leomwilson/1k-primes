package java1kp;

import java.util.ArrayList;

public class Java {
    public static void main(String[] args) {
	ArrayList<Integer> s = new ArrayList<Integer>();
	int p = 2;
	for (int i = 0; i < 1000; i++) {
	    boolean prime = true;
	    for (int j : s) {
		if (p % j == 0) {
		    prime = false;
		    break;
		}
	    }
	    if (prime) {
		s.add(p);
		System.out.println(p);
	    }
	    p++;
	}
    }
}
