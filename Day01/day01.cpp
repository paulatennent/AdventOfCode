#include <iostream>
#include <string>
#include <vector>
#include <set>
#include <map>
#include <algorithm>
#include <queue>
#include <stack>

#define MAX 200
#define TARGET 2020

using namespace std;

int arra[1000000];

// first sub question, find 2 numbers that add to the target
int a() {
    int a = 0;
    int b = MAX - 1;
    while (a < b) {
        if (arra[a] + arra[b] == TARGET) {
            return arra[a] * arra[b];
        } else if (arra[a] + arra[b] < TARGET) {
            a++;
        } else {
            b--;
        }
    }
    return -1;
}

// second sub question, find 2 numbers that add to the target
int b() {
    for (int a = 0; a < MAX; a++) {
        for (int b = 0; b < MAX; b++) {
            for (int c = 0; c < MAX; c++) {
                if (arra[a] + arra[b] + arra[c] == TARGET) {
                    return arra[a] * arra[b] * arra[c];
                }
            }
        }
    }
    return -1;
}

int main() {

    // input in data
    for (int i = 0; i < MAX; i++) {
        int data;
        cin >> data;
        arra[i] = data;
    }

    sort(arra, arra+MAX);

    printf("a: %d\n", a());
    printf("b: %d\n", b());

}