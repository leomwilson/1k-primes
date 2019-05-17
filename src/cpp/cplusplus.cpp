#include <iostream>

bool isPrime(int n) {
    for (int i = 2; i < n; i++) {
        if (n % i == 0) {
            return false;
        }
    }
    return true;
}

int main() {
    int p = 2;
    for (int i = 0; i < 1000; i++) {
        while (!isPrime(p)) {
            p++;
        }
        std::cout << p << std::endl;
        p++;
    }
}