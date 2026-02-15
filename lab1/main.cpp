#include <vector>
#include <random>
#include <cstdio>

int main() {
    std::mt19937 mt{};
    std::random_device random_device{};
    std::mt19937 gen(random_device());
    std::uniform_int_distribution<> distrib;
    std::vector<int> a{};
    printf("hello, yopta\n");
}
