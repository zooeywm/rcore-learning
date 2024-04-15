#include <stdio.h>
#include <unistd.h>

int main() {
    sleep(5);
    const char *hello_string = "Hello, Linux!\n";
    printf("%s", hello_string);
    FILE *output_file = fopen("output.txt", "w");
    fputs(hello_string, output_file);
    fclose(output_file);
    return 0;
}
