#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define MAX_BUFFER_SIZE 256
#define MAX_DATA_SIZE 100

struct Data {
    char message[MAX_DATA_SIZE];
};

// Shared global buffer.
char buffer[MAX_BUFFER_SIZE];
int buffer_length = 0;
pthread_mutex_t buffer_mutex = PTHREAD_MUTEX_INITIALIZER;

// Write data asynchronizely to shared buffer.
void *write_to_buffer(void *arg) {
    const char *message = (const char *)arg;
    // Lock the mutex, ensure safe access to the data.
    pthread_mutex_lock(&buffer_mutex);

    // Write data to buffer.
    int message_length = strlen(message);
    if (message_length <= MAX_BUFFER_SIZE - buffer_length) {
        strncpy(buffer + buffer_length, message, message_length);
        buffer_length += message_length;
        buffer[buffer_length] = '\0';
    } else {
        printf("Buffer overflow!\n");
    }

    pthread_mutex_unlock(&buffer_mutex);

    pthread_exit(NULL);
}

void write_to_file(const char *filename, struct Data *data) {
    FILE *file = fopen(filename, "wb");
    if (file == NULL) {
        perror("Error opening file.");
        return;
    }
    fwrite(data, sizeof(struct Data), 1, file);
    fclose(file);
}

void read_from_file(const char *filename, struct Data *data) {
    FILE *file = fopen(filename, "rb");
    if (file == NULL) {
        perror("Error opening file.");
        return;
    }
    fread(data, sizeof(struct Data), 1, file);
    fclose(file);
}

int main() {
    struct Data my_data;
    struct Data loaded_data;

    pthread_t thread1, thread2;

    pthread_create(&thread1, NULL, write_to_buffer, (void *)"Hello ");
    pthread_create(&thread2, NULL, write_to_buffer, (void *)"World!");

    pthread_join(thread1, NULL);
    pthread_join(thread2, NULL);

    strcpy(my_data.message, buffer);

    write_to_file("data.bin", &my_data);
    read_from_file("data.bin", &loaded_data);
    printf("Loaded message: %s\n", loaded_data.message);

    return 0;
}
