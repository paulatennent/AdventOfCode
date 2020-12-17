#include <iostream>
#include <string>
#include <vector>
#include <set>
#include <map>
#include <algorithm>
#include <queue>
#include <stack>

using namespace std;

int main() {
    int arra[1000000];

    for (int i = 0; i < 200; i++) {
        int data;
        cin >> data;
        arra[i] = data;
    }

    for (int a = 0; a < 200; a++) {
        for (int b = 0; b < 200; b++) {
            for (int c = 0; c < 200; c++) {
                if (arra[a] + arra[b] + arra[c] == 2020) {
                    printf("ANS: %d\n", arra[a] * arra[b] * arra[c]);
                }
            }
        }
    }
}