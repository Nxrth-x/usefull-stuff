#include <stdio.h>
#include <strings.h>

void encrypt(char *text, char *store, int key) {
	char encrypted[128];
	const int length = strlen(text);
	int i = 0;

	for(int i=0; i<length; i++) store[i] = (int)text[i] + key;
	store[length] = '\0';
}

int main(void) {
	// Variables
	char user_input[128], encrypted[128];
	int key;

	// User input
	printf("Text: ");
	fgets(user_input, sizeof(user_input), stdin);
	printf("Key (int): ");
	scanf("%d", &key);

	encrypt(user_input, encrypted, key);

	printf("Encrypted text: %s\n", encrypted);

	return 0;
}