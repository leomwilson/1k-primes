function isPrime(p) {
    for (var i = 2; i < p; i++) {
	if ((p % i) == 0) {
	    return false;
	}
    }
    return true;
}

var p = 2;

for (var i = 0; i < 1000; i++) {
    while (!isPrime(p)) {
	p++;
    }
    console.log(p);
    p++;
}
