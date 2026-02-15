import core.stdc.stdio;
import core.stdc.string;
import core.stdc.stdlib;

void println(string format, Args...)(Args args) {
    printf(format ~ "\n", args);
}

T* calloc(T)(size_t count) {
    return cast(T*)core.stdc.stdlib.calloc(count, T.sizeof);
}

struct Point {
    int x;
    int y;
}

extern(C) void main() {
    const size_t points_count = 3000;
    Point* points = calloc!(Point)(points_count);
    foreach(i; 0..points_count) {
        points[i].x = rand();
        points[i].y = rand();
    }
}
