//123456789012345678901234567890
int compare(char str0[32], char str1[32]) {
	int r=0;
	for (int i=32;i>0;i--) {
		r ^= str0[i] ^ str1[i];
	}
	return r;
}

int main() {
	char buf[4096] = {};
// todo read with fgets
	char lineBuf[32] = {};

	for (int i=0;i<4096; i++) {

	}

	return 0;
}
